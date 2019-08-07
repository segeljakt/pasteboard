use {
    cocoa::{
        appkit::{NSImage, NSPasteboard, NSSound},
        base::nil,
        foundation::{NSArray, NSAutoreleasePool, NSData, NSDictionary, NSString},
    },
    objc::{class, msg_send, runtime::Object, sel, sel_impl},
    structopt::clap::arg_enum,
};
pub type Id = *mut Object;

arg_enum! {
    #[derive(Debug)]
    pub enum Pasteboard {
        String,
        Image,
        Sound,
    }
}

impl Pasteboard {
    pub unsafe fn copy(&self, path: &str) {
        let pool = NSAutoreleasePool::new(nil);

        let path = NSString::alloc(pool).init_str(path);

        let data = NSData::dataWithContentsOfFile_(pool, path);

        let object = match self {
            Pasteboard::String => {
                let string: Id = msg_send![NSString::alloc(pool), initWithData:data encoding:4];
                string
            }
            Pasteboard::Image => NSImage::initWithData_(NSImage::alloc(pool), data),
            Pasteboard::Sound => {
                let alloc: Id = msg_send![class!(NSSound), alloc];
                NSSound::initWithData_(alloc, data)
            }
        };

        if object != nil {
            let pasteboard = NSPasteboard::generalPasteboard(pool);
            pasteboard.clearContents();
            pasteboard.writeObjects(NSArray::arrayWithObject(pool, object));
        }
    }

    pub unsafe fn paste(&self, path: &str) {
        let pool = NSAutoreleasePool::new(nil);

        let path = NSString::alloc(pool).init_str(path);

        let class = match self {
            Pasteboard::String => class!(NSString),
            Pasteboard::Image => class!(NSImage),
            Pasteboard::Sound => class!(NSSound),
        };

        let class_array = msg_send![class!(NSArray), arrayWithObject: class];
        let options = NSDictionary::dictionary(pool);

        let pasteboard = NSPasteboard::generalPasteboard(pool);
        if pasteboard.canReadObjectForClasses_options(class_array, options) != 0 {
            let objects_to_paste = pasteboard.readObjectsForClasses_options(class_array, options);
            let object = objects_to_paste.objectAtIndex(0);
            NSData::writeToFile_atomically_(object, path, 0);
        }
    }
}
