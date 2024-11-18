use crate::execution_engine::opcode::*;
use crate::heap::heap::with_heap_read_lock;
use crate::stack::stack_frame::StackFrame;
use tracing::trace;

pub(crate) fn process(code: u8, stack_frames: &mut Vec<StackFrame>) -> crate::error::Result<()> {
    let stack_frame = stack_frames.last_mut().unwrap();
    match code {
        ILOAD => {
            let pos = stack_frame.extract_one_byte() as usize;

            let value: i32 = stack_frame.get_local(pos);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("ILOAD -> pos={pos}, value={value}");
        }
        LLOAD => {
            let pos = stack_frame.extract_one_byte() as usize;
            let value: i64 = stack_frame.get_local(pos);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("LLOAD -> pos={pos}, value={value}");
        }
        FLOAD => {
            let pos = stack_frame.extract_one_byte() as usize;

            let value: f32 = stack_frame.get_local(pos);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("FLOAD -> pos={pos}, value={value}");
        }
        DLOAD => {
            let pos = stack_frame.extract_one_byte() as usize;
            let value: f64 = stack_frame.get_local(pos);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("DLOAD -> pos={pos}, value={value}");
        }
        ALOAD => {
            let index = stack_frame.extract_one_byte() as usize;

            let objectref: i32 = stack_frame.get_local(index);
            stack_frame.push(objectref);

            stack_frame.incr_pc();
            trace!("ALOAD -> index={index}, objectref={objectref}");
        }
        ILOAD_0 => {
            let value: i32 = stack_frame.get_local(0);
            stack_frame.push(value);
            stack_frame.incr_pc();
            trace!("ILOAD_0 -> value={value}");
        }
        ILOAD_1 => {
            let value: i32 = stack_frame.get_local(1);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("ILOAD_1 -> value={value}");
        }
        ILOAD_2 => {
            let value: i32 = stack_frame.get_local(2);
            stack_frame.push(value);
            stack_frame.incr_pc();
            trace!("ILOAD_2 -> value={value}");
        }
        ILOAD_3 => {
            let value: i32 = stack_frame.get_local(3);
            stack_frame.push(value);
            stack_frame.incr_pc();
            trace!("ILOAD_3 -> value={value}");
        }
        LLOAD_0 => {
            let value: i64 = stack_frame.get_local(0);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("LLOAD_0 -> value={value}");
        }
        LLOAD_1 => {
            let value: i64 = stack_frame.get_local(1);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("LLOAD_1 -> value={value}");
        }
        LLOAD_2 => {
            let value: i64 = stack_frame.get_local(2);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("LLOAD_2 -> value={value}");
        }
        LLOAD_3 => {
            let value: i64 = stack_frame.get_local(3);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("LLOAD_3 -> value={value}");
        }
        FLOAD_0 => {
            let value: f32 = stack_frame.get_local(0);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("FLOAD_0 -> value={value}");
        }
        FLOAD_1 => {
            let value: f32 = stack_frame.get_local(1);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("FLOAD_1 -> value={value}");
        }
        FLOAD_2 => {
            let value: f32 = stack_frame.get_local(2);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("FLOAD_2 -> value={value}");
        }
        FLOAD_3 => {
            let value: f32 = stack_frame.get_local(3);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("FLOAD_3 -> value={value}");
        }
        DLOAD_0 => {
            let value: f64 = stack_frame.get_local(0);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("DLOAD_0 -> value={value:e}");
        }
        DLOAD_1 => {
            let value: f64 = stack_frame.get_local(1);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("DLOAD_1 -> value={value:e}");
        }
        DLOAD_2 => {
            let value: f64 = stack_frame.get_local(2);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("DLOAD_2 -> value={value:e}");
        }
        DLOAD_3 => {
            let value: f64 = stack_frame.get_local(3);
            stack_frame.push(value);

            stack_frame.incr_pc();
            trace!("DLOAD_3 -> value={value:e}");
        }
        ALOAD_0 => {
            let reference: i32 = stack_frame.get_local(0);
            stack_frame.push(reference);
            stack_frame.incr_pc();
            trace!("ALOAD_0 -> reference={reference}");
        }
        ALOAD_1 => {
            let reference: i32 = stack_frame.get_local(1);
            stack_frame.push(reference);
            stack_frame.incr_pc();
            trace!("ALOAD_1 -> reference={reference}");
        }
        ALOAD_2 => {
            let reference: i32 = stack_frame.get_local(2);
            stack_frame.push(reference);
            stack_frame.incr_pc();
            trace!("ALOAD_2 -> reference={reference}");
        }
        ALOAD_3 => {
            let reference: i32 = stack_frame.get_local(3);
            stack_frame.push(reference);
            stack_frame.incr_pc();
            trace!("ALOAD_3 -> reference={reference}");
        }
        IALOAD => {
            let index = stack_frame.pop();
            let arrayref: i32 = stack_frame.pop();
            let value =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            stack_frame.push(value[0]);
            stack_frame.incr_pc();
            trace!(
                "IALOAD -> arrayref={arrayref}, index={index}, value={}",
                value[0]
            );
        }
        LALOAD => {
            let index: i32 = stack_frame.pop();
            let arrayref: i32 = stack_frame.pop();
            let value =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            let high = value[0];
            let low = value[1];

            stack_frame.push(low);
            stack_frame.push(high);
            stack_frame.incr_pc();
            trace!("LALOAD -> arrayref={arrayref}, index={index}, value={value:?}");
        }
        FALOAD => {
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();
            let value =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            stack_frame.push(value[0]);
            stack_frame.incr_pc();
            trace!(
                "FALOAD -> arrayref={arrayref}, index={index}, value={}",
                value[0]
            );
        }
        DALOAD => {
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();
            let value =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            let high = value[0];
            let low = value[1];

            stack_frame.push(low);
            stack_frame.push(high);
            stack_frame.incr_pc();
            trace!("DALOAD -> arrayref={arrayref}, index={index}, value={value:?}");
        }
        AALOAD => {
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();
            let objref =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            stack_frame.push(objref[0]);
            stack_frame.incr_pc();
            trace!(
                "AALOAD -> arrayref={arrayref}, index={index}, objref={}",
                objref[0]
            );
        }
        BALOAD => {
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();
            let value =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            stack_frame.push(value[0]);
            stack_frame.incr_pc();
            trace!(
                "BALOAD -> arrayref={arrayref}, index={index}, value={}",
                value[0]
            );
        }
        CALOAD => {
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();
            let value =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            stack_frame.push(value[0]);
            stack_frame.incr_pc();
            trace!(
                "CALOAD -> arrayref={arrayref}, index={index}, value={}",
                value[0]
            );
        }
        SALOAD => {
            let index = stack_frame.pop();
            let arrayref = stack_frame.pop();
            let value =
                with_heap_read_lock(|heap| heap.get_array_value(arrayref, index).cloned())?;

            stack_frame.push(value[0]);
            stack_frame.incr_pc();
            trace!(
                "SALOAD -> arrayref={arrayref}, index={index}, value={}",
                value[0]
            );
        }
        _ => {
            return Err(crate::error::Error::new_execution(&format!(
                "Unknown load opcode: {}",
                code
            )));
        }
    }

    Ok(())
}
