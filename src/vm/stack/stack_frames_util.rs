use crate::vm::error::{Error, Result};
use crate::vm::heap::heap::HEAP;
use crate::vm::jni::java_thread::JavaThread;
use crate::vm::method_area::instance_checker::InstanceChecker;
use crate::vm::method_area::java_method::JavaMethod;
use crate::vm::method_area::loaded_classes::CLASSES;
use derive_new::new;
use getset::CopyGetters;
use std::collections::BTreeMap;
use std::sync::Arc;

pub struct StackFramesUtil {}

const REFLECTION: &str = "jdk/internal/reflect/Reflection";
const GET_CALLER_CLASS: &str = "getCallerClass:()Ljava/lang/Class;";
const CALLER_SENSITIVE: &str = "Ljdk/internal/reflect/CallerSensitive;";
const LAMBDA_FORM_COMPILED: &str = "Ljava/lang/invoke/LambdaForm$Compiled;";
const METHOD: &str = "java/lang/reflect/Method";
const METHOD_INVOKE: &str = "invoke:(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;";
const METHOD_ACCESSOR_IMPL: &str = "jdk/internal/reflect/MethodAccessorImpl";
const PLATFORM_CLASS_LOADER: &str = "jdk/internal/loader/ClassLoaders$PlatformClassLoader";

/// What `Reflection.getCallerClass` finds on the stack.
#[derive(Debug, PartialEq)]
pub enum CallerClass {
    /// The class of the method that called the caller-sensitive method.
    Found(String),
    /// No frame left below the caller-sensitive method.
    NotFound,
    /// `Reflection.getCallerClass` was not called from a caller-sensitive method: the message of
    /// the `InternalError` HotSpot throws.
    Misuse(String),
}

#[derive(Debug, new, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct StackElement {
    class_ref: i32,
    method_ref: i64,
    line_number: u16,
    /// Start bytecode index of the instruction this frame was executing, used to build helpful
    /// `NullPointerException` messages (JEP 358). Zero for native frames.
    bci: u16,
    is_native: bool,
}

impl StackFramesUtil {
    /// Finds the caller of the caller-sensitive method that called `Reflection.getCallerClass`,
    /// walking the stack like HotSpot's `JVM_GetCallerClass`: frame 0 must be
    /// `Reflection.getCallerClass` and frames 0 and 1 caller-sensitive; the caller is the first
    /// frame after them that a security stack walk doesn't skip.
    pub fn get_caller_class_name() -> Result<CallerClass> {
        // Copied out first: checking the frames' methods may load classes, which must not happen
        // while the stack is borrowed. The walk usually stops within a few frames.
        let frames = JavaThread::with_frames(|frames| {
            frames
                .map(|frame| {
                    (
                        frame.current_class_name().to_string(),
                        frame.method_name().to_string(),
                    )
                })
                .collect::<Vec<_>>()
        })?;

        for (n, (class_name, method_name)) in frames.iter().enumerate() {
            let method = CLASSES.get(class_name)?.get_method(method_name)?;
            match n {
                0 if method_name != GET_CALLER_CLASS || class_name != REFLECTION => {
                    return Ok(CallerClass::Misuse(
                        "JVM_GetCallerClass must only be called from Reflection.getCallerClass"
                            .to_string(),
                    ));
                }
                0 | 1 => {
                    if !Self::is_caller_sensitive(class_name, &method)? {
                        return Ok(CallerClass::Misuse(format!(
                            "CallerSensitive annotation expected at frame {n}"
                        )));
                    }
                }
                _ => {
                    if !Self::is_ignored_by_security_stack_walk(class_name, &method)? {
                        return Ok(CallerClass::Found(class_name.clone()));
                    }
                }
            }
        }
        Ok(CallerClass::NotFound)
    }

    fn is_caller_sensitive(class_name: &str, method: &JavaMethod) -> Result<bool> {
        Ok(method.has_annotation(CALLER_SENSITIVE) && Self::is_privileged(class_name)?)
    }

    /// Whether a frame of `method` is reflection or method handle machinery, as HotSpot's
    /// `Method::is_ignored_by_security_stack_walk` decides.
    fn is_ignored_by_security_stack_walk(class_name: &str, method: &JavaMethod) -> Result<bool> {
        if class_name == METHOD && method.name_signature() == METHOD_INVOKE {
            return Ok(true);
        }
        if InstanceChecker::checkcast(class_name, METHOD_ACCESSOR_IMPL)? {
            return Ok(true);
        }
        Ok(method.is_polymorphic_signature()
            || (method.has_annotation(LAMBDA_FORM_COMPILED) && Self::is_privileged(class_name)?))
    }

    /// Whether `class_name` was defined by the bootstrap or platform class loader: HotSpot
    /// ignores `@CallerSensitive` and `@LambdaForm.Compiled` on any other class.
    fn is_privileged(class_name: &str) -> Result<bool> {
        let mirror_ref = CLASSES.get(class_name)?.mirror_clazz_ref()?;
        let loader_ref =
            HEAP.get_object_field_value(mirror_ref, "java/lang/Class", "classLoader")?[0];
        Ok(loader_ref == 0 || HEAP.get_instance_name(loader_ref)? == PLATFORM_CLASS_LOADER)
    }

    pub fn collect_stack_trace(
        throwable_name: &str,
        max_stack_size: usize,
    ) -> Result<Vec<StackElement>> {
        let mut stack_trace = JavaThread::with_frames(|frames| {
            // `with_frames` yields frames newest → oldest.
            let newest_to_oldest: Vec<_> = frames.collect();

            // Drop the `fillInStackTrace` machinery sitting at the top of the stack: the synthetic
            // native `fillInStackTrace(int)` frame plus the `Throwable.fillInStackTrace()` wrapper
            // that called it. They are always the newest frames while this native runs, and the JVM
            // never records them — the trace must start at the throw / `fillInStackTrace()` call
            // site. This matters when `fillInStackTrace()` is invoked directly on an already
            // constructed throwable: there is then no `<init>` frame for the `throwable_name` skip
            // below to stop at, so without this the synthetic frames would leak in as the top
            // elements. In the normal construction path these same frames sit above the `<init>`
            // frames and are already excluded by the `throwable_name` skip, so this is a no-op there.
            // `method_name()` carries the `name:descriptor` form, so match on the bare name.
            let after_fill = newest_to_oldest
                .iter()
                .position(|frame| {
                    let name = frame.method_name().split(':').next();
                    !(frame.current_class_name() == "java/lang/Throwable"
                        && name == Some("fillInStackTrace"))
                })
                .unwrap_or(newest_to_oldest.len());

            // Reverse to oldest → newest so the throwable-skip stops at the throwable's own
            // constructor, excluding it together with its superclass `<init>` frames sitting above.
            let eligible: Vec<_> = newest_to_oldest[after_fill..]
                .iter()
                .rev()
                .take_while(|frame| frame.current_class_name() != throwable_name)
                .collect();

            // When the stack is deeper than `max_stack_size`, the JVM keeps the *newest* frames
            // (the throw site and its nearest callers) and truncates the oldest, so take the tail
            // of this oldest → newest slice. Resolving elements only for the retained frames avoids
            // wasted lookups — and error propagation — for frames that would be dropped.
            let start = eligible.len().saturating_sub(max_stack_size);
            let mut stack_trace = Vec::with_capacity(eligible.len() - start);
            for frame in &eligible[start..] {
                let klass = CLASSES.get(frame.current_class_name())?;
                let method_name = frame.method_name();
                let method = klass.get_method(method_name)?;
                let method_raw = Arc::as_ptr(&method) as i64;

                let pc = frame.ex_pc();
                let line_numbers = frame.line_numbers();
                let instruction_line_num = Self::extract_line_number(line_numbers, pc);

                let class_ref = klass.mirror_clazz_ref()?;
                let native = method.is_native();

                stack_trace.push(StackElement::new(
                    class_ref,
                    method_raw,
                    instruction_line_num,
                    frame.bci() as u16,
                    native,
                ));
            }

            Ok::<_, Error>(stack_trace)
        })??;

        stack_trace.reverse();
        Ok(stack_trace)
    }

    /// Collects the calling thread's full stack, newest frame first (index 0 = the currently
    /// executing method), up to `max_stack_size` frames. Unlike [`Self::collect_stack_trace`] this
    /// applies no throwable-specific frame skipping — it is used to snapshot an arbitrary thread's
    /// stack (e.g. `Thread.getStackTrace`) at a safepoint, where the newest frame is wherever the
    /// thread was paused.
    pub fn collect_current_stack(max_stack_size: usize) -> Result<Vec<StackElement>> {
        JavaThread::with_frames(|frames| {
            let mut elements = Vec::new();
            for frame in frames {
                if elements.len() >= max_stack_size {
                    break;
                }
                let klass = CLASSES.get(frame.current_class_name())?;
                let method = klass.get_method(frame.method_name())?;
                let method_raw = Arc::as_ptr(&method) as i64;
                // Use the call-site PC (`ex_pc`), like `collect_stack_trace`: for a caller frame the
                // live `pc` has already advanced past the invoke, so it would report the line after
                // the call rather than the call site.
                let line = Self::extract_line_number(frame.line_numbers(), frame.ex_pc());
                let class_ref = klass.mirror_clazz_ref()?;
                elements.push(StackElement::new(
                    class_ref,
                    method_raw,
                    line,
                    frame.bci() as u16,
                    method.is_native(),
                ));
            }
            Ok::<_, Error>(elements)
        })?
    }

    pub fn extract_line_number(line_numbers: &BTreeMap<u16, u16>, pc: usize) -> u16 {
        let instruction_line_num = line_numbers
            .range(..=&(pc as u16))
            .next_back()
            .map(|(_pc, line)| *line)
            .unwrap_or_default();
        instruction_line_num
    }
}
