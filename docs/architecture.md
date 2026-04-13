# Architecture of rusty-jvm

This document describes the internal architecture of `rusty-jvm` - a Java Virtual Machine
implemented from scratch in Rust, targeting the JVM Specification SE 25.

---

## 1. Top-Level Module Map

The VM is split into a set of tightly-scoped modules.
Each module owns exactly one piece of VM state and exposes a narrow, well-typed interface to the rest of the system.

```mermaid
graph TD
    CLI["CLI / lib.rs<br/>(Arguments, run())"]
    VM["vm/mod.rs<br/>(bootstrap & wiring)"]
    MA["method_area<br/>(class loading, vtable)"]
    HEAP["heap<br/>(object & array storage)"]
    EE["execution_engine<br/>(bytecode interpreter)"]
    STACK["stack<br/>(operand stack / frames)"]
    JNI["jni<br/>(JNI ABI bridge)"]
    SN["system_native<br/>(native method impls)"]
    PROPS["properties<br/>(system props / classpath)"]
    EXC["exception<br/>(exception creation & propagation)"]
    VALID["validation<br/>(bytecode pre-checks)"]

    CLI --> VM
    VM --> MA
    VM --> HEAP
    VM --> EE
    EE --> MA
    EE --> HEAP
    EE --> STACK
    EE --> SN
    EE --> EXC
    SN --> JNI
    JNI --> HEAP
    JNI --> MA
    MA --> PROPS
    VM --> PROPS
    EE --> VALID
```

---

## 2. Class-Loading Pipeline

Classes are loaded on first reference (lazy loading) and cached forever in `CLASSES` - a global
`DashMap` keyed by the JVM internal class name (e.g. `java/lang/String`).

```mermaid
sequenceDiagram
    participant EE as ExecutionEngine
    participant MA as MethodArea
    participant JI as JImage (JDK rt)
    participant FS as FileSystem (user .class)
    participant CL as CLASSES (cache)

    EE->>MA: load_class("com/example/Foo")
    MA->>CL: already cached?
    alt cached
        CL-->>MA: Arc<JavaClass>
    else not cached
        MA->>JI: find_resource("/java.base/com/example/Foo.class")
        alt found in JImage
            JI-->>MA: raw bytes
        else not in JImage
            MA->>FS: open("com/example/Foo.class")
            FS-->>MA: raw bytes
        end
        MA->>MA: parse (jclassfile), build JavaClass
        MA->>CL: insert Arc<JavaClass>
        CL-->>MA: Arc<JavaClass>
    end
    MA-->>EE: Arc<JavaClass>
```

---

## 3. Heap Memory Model

The heap is a `DashMap<i32, HeapValue>`.
Every object reference in the VM is a plain `i32` (matching the JVM operand-stack word size).
References start at 1; 0 is the JVM null.

```mermaid
classDiagram
    class Heap {
        -data: AutoDashMapI32~HeapValue~
        -ref_by_stringvalue: DashMap~String,i32~
        +create_instance(JavaInstance) i32
        +create_array(type_name, len) i32
        +get_object_field_value(objectref, class, field) Result~Vec~i32~~
        +set_object_field_value(objectref, class, field, value)
        +get_array_value(arrayref, index) Result~Vec~i32~~
        +set_array_value(arrayref, index, value)
    }

    class HeapValue {
        <<enum>>
        Object(JavaInstance)
        Array(Array)
    }

    class JavaInstance {
        -class_name: String
        -fields: HashMap~FieldKey, Vec~i32~~
    }

    class Array {
        -type_name: String
        -data: Vec~u8~
    }

    Heap "1" *-- "N" HeapValue
    HeapValue --> JavaInstance
    HeapValue --> Array
```

---

## 4. Bytecode Execution Loop

The interpreter is a single `while` loop over a `StackFrames` stack.
Each iteration reads one bytecode, delegates to an opcode-family processor, then loops.
Method calls push a new frame; returns pop it.

```mermaid
flowchart TD
    START([Engine::execute]) --> PUSH[Push initial StackFrame]
    PUSH --> LOOP{stack_frames empty?}
    LOOP -- no --> READ[Read bytecode at PC]
    READ --> DISPATCH{opcode range}
    DISPATCH -- 0-20 constants --> CONST[ops_constant_processor]
    DISPATCH -- 21-53 loads --> LOAD[ops_load_processor]
    DISPATCH -- 54-86 stores --> STORE[ops_store_processor]
    DISPATCH -- 87-95 stack ops --> STACKOP[ops_stack_processor]
    DISPATCH -- 96-131 math --> MATH[ops_math_processor]
    DISPATCH -- 133-147 conversions --> CONV[ops_conversion_processor]
    DISPATCH -- 148-166 comparisons --> CMP[ops_comparison_processor]
    DISPATCH -- 167-177 control --> CTRL[ops_control_processor]
    DISPATCH -- 178-195 references --> REF[ops_reference_processor]
    DISPATCH -- 196-201 extended --> EXT[ops_extended_processor]
    CONST & LOAD & STORE & STACKOP & MATH & CONV & CMP & CTRL & REF & EXT --> LOOP
    LOOP -- yes --> DONE([return last_value])
```

---

## 5. Virtual Method Dispatch (vtable)

Each `JavaClass` holds a lazily-initialized vtable: an `OnceCell<IndexMap<String, Arc<JavaMethod>>>`.
The vtable is built once by `MethodArea::build_vtable`, which follows the order mandated by JVMS §5.4.5:
parent vtable → interface default methods → own concrete methods.

```mermaid
flowchart TD
    INVOKE[invokevirtual / invokeinterface] --> LOOKUP[lookup_for_implementation\nclass_name + method_sig]
    LOOKUP --> VTABLE{vtable initialised?}
    VTABLE -- no --> BUILD[build_vtable:\n1. inherit parent vtable\n2. overlay interface defaults\n3. insert own methods]
    BUILD --> STORE_VT[store in OnceCell]
    STORE_VT --> HIT
    VTABLE -- yes --> HIT["O(1) IndexMap lookup"]
    HIT --> METHOD[Arc~JavaMethod~]
    METHOD --> FRAME[Push new StackFrame\nwith method bytecode]
    FRAME --> EE[continue execution loop]
```

---

## 6. JNI Bridge

Native methods declared in Java class files are dispatched through a two-layer bridge:
a static lookup table that maps JVM internal method names to Rust function pointers,
and a `libffi`-based dynamic dispatcher for methods loaded from external `.so`/`.dll` libraries.

```mermaid
sequenceDiagram
    participant EE as ExecutionEngine
    participant SNT as system_native_table (static map)
    participant DISP as libffi Dispatcher
    participant NLIB as Native .so / .dll
    participant JNI as JNI ABI layer

    EE->>SNT: invoke_native(class, method, args)
    alt found in static table
        SNT-->>EE: call Rust fn directly
    else dynamic native
        EE->>DISP: build_args (i32 refs → *mut c_void)
        DISP->>NLIB: ffi_call(native_fn, args)
        NLIB->>JNI: JNIEnv* callbacks (FindClass, GetMethodID, …)
        JNI-->>NLIB: results
        NLIB-->>DISP: return value
        DISP-->>EE: Vec<i32>
    end
```
