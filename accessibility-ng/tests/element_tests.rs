use accessibility_ng::{AXAttribute, AXUIElement, Error, TreeVisitor, TreeWalker, TreeWalkerFlow};
use core_foundation::{base::TCFType, base::CFType, string::CFString};
use std::cell::Cell;

#[test]
fn test_system_wide_element_creation() {
    let element = AXUIElement::system_wide();
    assert!(!element.as_CFTypeRef().is_null());
    
    // Check we can get attribute names
    let names_result = element.attribute_names();
    
    // This shouldn't fail on any macOS system
    assert!(names_result.is_ok());
    
    let names = names_result.unwrap();
    assert!(!names.is_empty());
}

#[test]
fn test_application_is_trusted_is_callable() {
    // Just check that these don't crash
    let _ = AXUIElement::application_is_trusted();
    
    // We're not invoking the prompt version in the test because
    // it would show a UI dialog, but we can check it exists
    let _trusted_fn_ptr = AXUIElement::application_is_trusted as fn() -> bool;
    // Simply check it's defined
    assert!(true);
}

#[test]
fn test_attribute_creation() {
    let attr_children = AXAttribute::children();
    assert_eq!(attr_children.as_CFString().to_string(), "AXChildren");
    
    let attr_title = AXAttribute::title();
    assert_eq!(attr_title.as_CFString().to_string(), "AXTitle");
    
    // Create a custom attribute using CFType (as that's what new() is implemented for)
    let custom_str = CFString::new("AXCustomAttribute");
    let attr_custom = AXAttribute::<CFType>::new(&custom_str);
    assert_eq!(attr_custom.as_CFString().to_string(), "AXCustomAttribute");
}

struct TestVisitor {
    count: Cell<usize>,
}

impl TestVisitor {
    fn new() -> Self {
        Self { count: Cell::new(0) }
    }
}

impl TreeVisitor for TestVisitor {
    fn enter_element(&self, _element: &AXUIElement) -> TreeWalkerFlow {
        self.count.set(self.count.get() + 1);
        TreeWalkerFlow::Continue
    }
    
    fn exit_element(&self, _element: &AXUIElement) {
        // Nothing to do here
    }
}

#[test]
fn test_tree_walker() {
    let element = AXUIElement::system_wide();
    let visitor = TestVisitor::new();
    let walker = TreeWalker::new();
    
    walker.walk(&element, &visitor);
    
    // At minimum, the system-wide element itself should be visited
    assert!(visitor.count.get() >= 1);
}

#[test]
fn test_error_handling() {
    let element = AXUIElement::system_wide();
    
    // Create a custom attribute for a non-existent attribute
    let nonexistent_str = CFString::new("AXNonExistentAttribute");
    let attr_nonexistent = AXAttribute::<CFType>::new(&nonexistent_str);
    let result = element.attribute(&attr_nonexistent);
    
    assert!(result.is_err());
    match result.unwrap_err() {
        Error::Ax(_) => {}, // This is expected
        other => panic!("Expected Ax error, got: {:?}", other),
    }
}