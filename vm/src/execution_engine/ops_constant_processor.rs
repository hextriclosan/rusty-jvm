use crate::execution_engine::opcode::*;
use crate::method_area::method_area::with_method_area;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(
    code: u8,
    current_class_name: &str,
    stack_frames: &mut Vec<StackFrame>,
) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        ACONST_NULL => {
            stack_frame.push(0);
            stack_frame.incr_pc();
            trace!("ACONST_NULL");
        }
        ICONST_M1 => {
            stack_frame.push(-1);
            stack_frame.incr_pc();
            trace!("ICONST_M1");
        }
        ICONST_0 => {
            stack_frame.push(0);
            stack_frame.incr_pc();
            trace!("ICONST_0");
        }
        ICONST_1 => {
            stack_frame.push(1);
            stack_frame.incr_pc();
            trace!("ICONST_1");
        }
        ICONST_2 => {
            stack_frame.push(2);
            stack_frame.incr_pc();
            trace!("ICONST_2");
        }
        ICONST_3 => {
            stack_frame.push(3);
            stack_frame.incr_pc();
            trace!("ICONST_3");
        }
        ICONST_4 => {
            stack_frame.push(4);
            stack_frame.incr_pc();
            trace!("ICONST_4");
        }
        ICONST_5 => {
            stack_frame.push(5);
            stack_frame.incr_pc();
            trace!("ICONST_5");
        }
        LCONST_0 => {
            stack_frame.push_i64(0i64);
            stack_frame.incr_pc();
            trace!("LCONST_0");
        }
        LCONST_1 => {
            stack_frame.push_i64(1i64);
            stack_frame.incr_pc();
            trace!("LCONST_1");
        }
        FCONST_0 => {
            stack_frame.push_f32(0.0);
            stack_frame.incr_pc();
            trace!("FCONST_0");
        }
        FCONST_1 => {
            stack_frame.push_f32(1.0);
            stack_frame.incr_pc();
            trace!("FCONST_1");
        }
        FCONST_2 => {
            stack_frame.push_f32(2.0);
            stack_frame.incr_pc();
            trace!("FCONST_2");
        }
        DCONST_0 => {
            stack_frame.push_f64(0.0);
            stack_frame.incr_pc();
            trace!("DCONST_0");
        }
        DCONST_1 => {
            stack_frame.push_f64(1.0);
            stack_frame.incr_pc();
            trace!("DCONST_1");
        }
        BIPUSH => {
            stack_frame.incr_pc();
            let value = stack_frame.get_bytecode_byte() as i8 as i32;
            stack_frame.push(value);
            stack_frame.incr_pc();
            trace!("BIPUSH -> value={value}");
        }
        SIPUSH => {
            let value = stack_frame.extract_two_bytes() as i32;
            stack_frame.push(value);
            stack_frame.incr_pc();
            trace!("SIPUSH -> value={value}");
        }
        LDC => {
            stack_frame.incr_pc();
            let cpoolindex = stack_frame.get_bytecode_byte() as u16;

            let value = with_method_area(|method_area| {
                method_area.resolve_ldc(&current_class_name, cpoolindex)
            })?;

            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("LDC -> cpoolindex={cpoolindex}, value={value}");
        }
        LDC_W => {
            //todo: merge me with LDC
            let cpoolindex = stack_frame.extract_two_bytes() as u16;

            let value = with_method_area(|method_area| {
                method_area.resolve_ldc(&current_class_name, cpoolindex)
            })?;

            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("LDC_W -> cpoolindex={cpoolindex}, value={value}");
        }
        LDC2_W => {
            //todo: merge me with LDC
            let cpoolindex = stack_frame.extract_two_bytes() as u16;

            let value = with_method_area(|method_area| {
                method_area.resolve_ldc2_w(&current_class_name, cpoolindex)
            })?;

            stack_frame.push_i64(value);

            stack_frame.incr_pc();
            trace!(
                "LDC2_W -> cpoolindex={cpoolindex}, value={value} or {:e}",
                f64::from_bits(value as u64)
            );
        }
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
                "Unknown constant opcode: {}",
                code
            )));
        }
    }

    Ok(())
}
