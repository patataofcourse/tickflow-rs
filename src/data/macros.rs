#[macro_export]
macro_rules! tf_op_args {
    ($cmdname:literal $(<$arg0:literal>)?, [$(($argnum:literal$(, $special:literal)?)),* $(,)?] $(, $scene:literal)? $(,)?) => {
        {
        #[allow(unused_mut, unused_assignments)]
        let mut arg0 = None;
        $(arg0 = Some($arg0);)?

        #[allow(unused_mut, unused_assignments)]
        let mut scene = -1;
        $(scene = $scene;)?

        #[allow(unused_mut)]
        let mut args = vec![];
        $(
            #[allow(unused_mut)]
            let mut val = ($argnum, false);
            $(val.1 = $special;)?
            args.push(val);
        )*

        $crate::data::ArgsTickflowOpDef {
            op: $cmdname,
            arg0,
            args,
            scene,
            }
        }
    };

}

#[macro_export]
macro_rules! tf_op {
    ($cmdname:literal <$arg0:literal> $(, $scene:literal)? $(,)?) => {
        $crate::data::TickflowOpDef {
            op: $cmdname,
            arg0: Some($arg0),
            scene: $($scene + 1)? - 1,
        }
    };
    ($cmdname:literal <=$arg0:pat=> $(, $scene:literal)? $(,)?) => {
        $crate::data::TickflowOpDef {
            op: $cmdname,
            arg0: Some($arg0),
            scene: $($scene + 1)? - 1,
        }
    };
    ($cmdname:literal $(, $scene:literal)? $(,)?) => {
        $crate::data::TickflowOpDef {
            op: $cmdname,
            arg0: None,
            scene: $($scene + 1)? - 1,
        }
    };
}
