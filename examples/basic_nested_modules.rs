mod mymodule {
    pub fn myfunc(){}
    pub mod mysubmodule{
        pub fn myotherfunc(){}
    }
}

fn main(){
    mymodule::myfunc();
    mymodule::mysubmodule::myotherfunc();
}
