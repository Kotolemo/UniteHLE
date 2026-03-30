/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! UIToolbar

use crate::frameworks::foundation::NSInteger;
use crate::objc::{id, impl_HostObject_with_superclass, objc_classes, ClassExports, NSZonePtr};

pub struct UIToolbarHostObject {
    superclass: super::ui_view::UIViewHostObject,
}
impl_HostObject_with_superclass!(UIToolbarHostObject);

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation UIToolbar: UIView

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_obj = Box::new(UIToolbarHostObject {
        superclass: Default::default(),
    });
    env.objc.alloc_object(this, host_obj, &mut env.mem)
}

- (id)init {
    this
}

- (id)initWithFrame:(crate::frameworks::core_graphics::CGRect)_frame {
    this
}

- (())setItems:(id)_items {
}

- (())setItems:(id)_items animated:(bool)_animated {
}

- (id)items {
    crate::objc::nil
}

- (())setBarStyle:(NSInteger)_style {
}

- (NSInteger)barStyle {
    0
}

- (())setTranslucent:(bool)_translucent {
}

- (bool)isTranslucent {
    false
}

- (())setTintColor:(id)_color {
}

- (id)tintColor {
    crate::objc::nil
}

- (())setBarTintColor:(id)_color {
}

@end

};
