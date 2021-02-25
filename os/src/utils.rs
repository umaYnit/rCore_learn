
#[macro_export]
macro_rules! arr_not_in_scope {
    ($start:expr,$len:expr, $( [$scope_start:expr,$scope_end:expr] ),+ $(,)?) => {{
        let end = $start+$len;
         true $( && ($start < $scope_start || end > $scope_end)  )+
   }}
}