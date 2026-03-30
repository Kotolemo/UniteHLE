//! UIBarButtonItem
//!
//! Minimal stub to prevent crash on alloc/init.

use crate::frameworks::foundation::NSInteger;
use crate::objc::{id, nil, ClassExports, HostObject};
use crate::objc_classes;

struct UIBarButtonItemHostObject;
impl HostObject for UIBarButtonItemHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIBarButtonItem: NSObject

+ (id)alloc {
    let host_obj = Box::new(UIBarButtonItemHostObject);
    env.objc.alloc_object(this, host_obj, &mut env.mem)
}

- (id)init {
    this
}

- (id)initWithBarButtonSystemItem:(NSInteger)_system_item
                           target:(id)_target
                           action:(u32)_action {
    this
}

- (id)initWithTitle:(id)_title
              style:(NSInteger)_style
             target:(id)_target
             action:(u32)_action {
    this
}

- (id)initWithImage:(id)_image
              style:(NSInteger)_style
             target:(id)_target
             action:(u32)_action {
    this
}

- (id)initWithCustomView:(id)_custom_view {
    this
}

- (())setEnabled:(bool)_enabled {
}

- (())setAction:(u32)_action {
}

- (())setTarget:(id)_target {
}

- (())setTitle:(id)_title {
}

- (())setStyle:(NSInteger)_style {
}

- (())setImage:(id)_image {
}

- (())setTintColor:(id)_color {
}

- (())setWidth:(f32)_width {
}

- (id)title {
    nil
}

- (bool)isEnabled {
    true
}

@end

};

