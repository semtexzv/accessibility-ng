use accessibility_sys_ng::*;

#[test]
fn test_constants_are_defined() {
    // Test that role constants are defined
    assert!(kAXButtonRole.as_ptr() != std::ptr::null());
    assert!(kAXCheckBoxRole.as_ptr() != std::ptr::null());
    assert!(kAXWindowRole.as_ptr() != std::ptr::null());
    
    // Test that attribute constants are defined
    assert!(kAXTitleAttribute.as_ptr() != std::ptr::null());
    assert!(kAXChildrenAttribute.as_ptr() != std::ptr::null());
    assert!(kAXParentAttribute.as_ptr() != std::ptr::null());
    
    // Test that action constants are defined
    assert!(kAXPressAction.as_ptr() != std::ptr::null());
    assert!(kAXIncrementAction.as_ptr() != std::ptr::null());
    assert!(kAXDecrementAction.as_ptr() != std::ptr::null());
}

#[test]
fn test_system_wide_element_creation() {
    unsafe {
        let element = AXUIElementCreateSystemWide();
        assert!(!element.is_null());
        
        // Test that we can get the element's type ID
        let type_id = AXUIElementGetTypeID();
        assert!(type_id > 0);
    }
}

// This test doesn't actually test trusted status but ensures the function is callable
#[test]
fn test_is_process_trusted_is_callable() {
    unsafe {
        // Just ensuring this doesn't crash
        let _ = AXIsProcessTrusted();
    }
}

#[test]
fn test_error_codes() {
    // Just check that error codes are defined
    assert!(kAXErrorSuccess != kAXErrorFailure);
    assert!(kAXErrorFailure != kAXErrorIllegalArgument);
    assert!(kAXErrorIllegalArgument != kAXErrorInvalidUIElement);
    
    // Check that error_string function returns something for different codes
    let failure_str = error_string(kAXErrorFailure);
    assert!(!failure_str.is_empty());
    
    let success_str = error_string(kAXErrorSuccess);
    assert!(!success_str.is_empty());
}