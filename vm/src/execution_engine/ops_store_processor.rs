use crate::execution_engine::opcode::*;
use crate::heap::heap::with_heap_write_lock;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut Vec<StackFrame>) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        ISTORE => {
            let pos = stack_frame.extract_one_byte() as usize;
            let value: i32 = stack_frame.pop();

            stack_frame.set_local(pos, value);
            stack_frame.incr_pc();
            trace!("ISTORE -> pos={pos}, value={value}");
        }
        LSTORE => {
            let pos = stack_frame.extract_one_byte() as usize;
            let value: i64 = stack_frame.pop();
            stack_frame.set_local(pos, value);

            stack_frame.incr_pc();
            trace!("LSTORE -> value={value}");
        }
        FSTORE => {
            let pos = stack_frame.extract_one_byte() as usize;
            let value: f32 = stack_frame.pop();

            stack_frame.set_local(pos, value);
            stack_frame.incr_pc();
            trace!("FSTORE -> pos={pos}, value={value}");
        }
        DSTORE => {
            let pos = stack_frame.extract_one_byte() as usize;
            let value: f64 = stack_frame.pop();
            stack_frame.set_local(pos, value);

            stack_frame.incr_pc();
            trace!("DSTORE -> value={value}");
        }
        ASTORE => {
            let index = stack_frame.extract_one_byte() as usize;
            let objectref: i32 = stack_frame.pop();

            stack_frame.set_local(index, objectref);

            stack_frame.incr_pc();
            trace!("ASTORE -> index={index}, objectref={objectref}");
        }
        ISTORE_0 => {
            let value: i32 = stack_frame.pop();
            stack_frame.set_local(0, value);

            stack_frame.incr_pc();
            trace!("ISTORE_0 -> value={value}");
        }
        ISTORE_1 => {
            let value: i32 = stack_frame.pop();
            stack_frame.set_local(1, value);

            stack_frame.incr_pc();
            trace!("ISTORE_1 -> value={value}");
        }
        ISTORE_2 => {
            let value: i32 = stack_frame.pop();
            stack_frame.set_local(2, value);

            stack_frame.incr_pc();
            trace!("ISTORE_2 -> value={value}");
        }
        ISTORE_3 => {
            let value: i32 = stack_frame.pop();
            stack_frame.set_local(3, value);

            stack_frame.incr_pc();
            trace!("ISTORE_3 -> value={value}");
        }
        LSTORE_0 => {
            let value: i64 = stack_frame.pop();
            stack_frame.set_local(0, value);

            stack_frame.incr_pc();
            trace!("LSTORE_0 -> value={value}");
        }
        LSTORE_1 => {
            let value: i64 = stack_frame.pop();
            stack_frame.set_local(1, value);

            stack_frame.incr_pc();
            trace!("LSTORE_1 -> value={value}");
        }
        LSTORE_2 => {
            let value: i64 = stack_frame.pop();
            stack_frame.set_local(2, value);

            stack_frame.incr_pc();
            trace!("LSTORE_2 -> value={value}");
        }
        LSTORE_3 => {
            let value: i64 = stack_frame.pop();
            stack_frame.set_local(3, value);

            stack_frame.incr_pc();
            trace!("LSTORE_3 -> value={value}");
        }
        FSTORE_0 => {
            let value: f32 = stack_frame.pop();
            stack_frame.set_local(0, value);

            stack_frame.incr_pc();
            trace!("FSTORE_0 -> value={value}");
        }
        FSTORE_1 => {
            let value: f32 = stack_frame.pop();
            stack_frame.set_local(1, value);

            stack_frame.incr_pc();
            trace!("FSTORE_1 -> value={value}");
        }
        FSTORE_3 => {
            let value: f32 = stack_frame.pop();
            stack_frame.set_local(3, value);

            stack_frame.incr_pc();
            trace!("FSTORE_3 -> value={value}");
        }
        DSTORE_0 => {
            let value: f64 = stack_frame.pop();
            stack_frame.set_local(0, value);

            stack_frame.incr_pc();
            trace!("DSTORE_0 -> value={value:e}");
        }
        DSTORE_1 => {
            let value: f64 = stack_frame.pop();
            stack_frame.set_local(1, value);

            stack_frame.incr_pc();
            trace!("DSTORE_1 -> value={value:e}");
        }
        DSTORE_2 => {
            let value: f64 = stack_frame.pop();
            stack_frame.set_local(2, value);

            stack_frame.incr_pc();
            trace!("DSTORE_2 -> value={value:e}");
        }
        DSTORE_3 => {
            let value: f64 = stack_frame.pop();
            stack_frame.set_local(3, value);

            stack_frame.incr_pc();
            trace!("DSTORE_3 -> value={value:e}");
        }
        ASTORE_0 => {
            let objectref: i32 = stack_frame.pop();
            stack_frame.set_local(0, objectref);

            stack_frame.incr_pc();
            trace!("ASTORE_0 -> objectref={objectref}");
        }
        ASTORE_1 => {
            let objectref: i32 = stack_frame.pop();
            stack_frame.set_local(1, objectref);

            stack_frame.incr_pc();
            trace!("ASTORE_1 -> objectref={objectref}");
        }
        ASTORE_2 => {
            let objectref: i32 = stack_frame.pop();
            stack_frame.set_local(2, objectref);

            stack_frame.incr_pc();
            trace!("ASTORE_2 -> objectref={objectref}");
        }
        ASTORE_3 => {
            let objectref: i32 = stack_frame.pop();
            stack_frame.set_local(3, objectref);

            stack_frame.incr_pc();
            trace!("ASTORE_3 -> objectref={objectref}");
        }
        IASTORE => {
            let value = stack_frame.pop();
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, vec![value]))?;

            stack_frame.incr_pc();
            trace!("IASTORE -> arrayref={arrayref}, index={index}, value={value}");
        }
        LASTORE => {
            let high = stack_frame.pop();
            let low = stack_frame.pop();

            let value = vec![high, low];
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, value.clone()))?;

            stack_frame.incr_pc();
            trace!("LASTORE -> arrayref={arrayref}, index={index}, value={value:?}");
        }
        FASTORE => {
            let value = stack_frame.pop();
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, vec![value]))?;

            stack_frame.incr_pc();
            trace!("FASTORE -> arrayref={arrayref}, index={index}, value={value}");
        }
        DASTORE => {
            let high = stack_frame.pop();
            let low = stack_frame.pop();

            let value = vec![high, low];
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, value.clone()))?;

            stack_frame.incr_pc();
            trace!("DASTORE -> arrayref={arrayref}, index={index}, value={value:?}");
        }
        AASTORE => {
            let objref = stack_frame.pop();
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, vec![objref]))?;

            stack_frame.incr_pc();
            trace!("AASTORE -> arrayref={arrayref}, index={index}, objref={objref}");
        }
        BASTORE => {
            let value = stack_frame.pop();
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, vec![value]))?;

            stack_frame.incr_pc();
            trace!("BASTORE -> arrayref={arrayref}, index={index}, value={value}");
        }
        CASTORE => {
            let value = stack_frame.pop();
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, vec![value]))?;

            stack_frame.incr_pc();
            trace!("CASTORE -> arrayref={arrayref}, index={index}, value={value}");
        }
        SASTORE => {
            let value = stack_frame.pop();
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();

            with_heap_write_lock(|heap| heap.set_array_value(arrayref, index, vec![value]))?;

            stack_frame.incr_pc();
            trace!("SASTORE -> arrayref={arrayref}, index={index}, value={value}");
        }
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
                "Unknown store opcode: {}",
                code
            )));
        }
    }

    Ok(())
}
