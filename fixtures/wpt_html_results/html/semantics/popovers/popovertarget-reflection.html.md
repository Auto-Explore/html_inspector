# html/semantics/popovers/popovertarget-reflection.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popovertarget-reflection.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1523410">
<link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1879001">
<link rel=help href="https://html.spec.whatwg.org/multipage/common-dom-interfaces.html#reflecting-content-attributes-in-idl-attributes:element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<button id=mybutton popovertarget="mypopover">toggle popover</button>
<div id=mypopover popover=auto>popover</div>

<script>
test(() => {
  assert_equals(mybutton.popoverTargetElement.id, "mypopover",
    'Setting element.popoverTargetElement to a valid element should work');

  mybutton.popoverTargetElement = null;
  assert_false(mybutton.hasAttribute('popovertarget'),
    'Setting element.popoverTargetElement to null should unset popovertarget attribute.');
  assert_equals(mybutton.popoverTargetElement, null,
    'Setting element.popoverTargetElement to null should remove the existing element from element.popoverTargetElement.');

  mybutton.popoverTargetElement = mypopover;
  assert_true(mybutton.hasAttribute('popovertarget'),
    'Assigning to element.popoverTargetElement should set the popovertarget attribute.');

  mybutton.removeAttribute('popovertarget');
  assert_equals(mybutton.popoverTargetElement, null,
    'Removing the popovertarget attribute should remove the element from element.popoverTargetElement.');

  mybutton.popoverTargetElement = mypopover;
  assert_true(mybutton.hasAttribute('popovertarget'),
    'Assigning to element.popoverTargetElement should set the popovertarget attribute.');

  mybutton.setAttribute("popovertarget", 'invalid');
  assert_equals(mybutton.popoverTargetElement, null,
    'Setting the popovertarget attribute to a localName that is not attr should remove the existing element from element.popoverTargetElement.');

  mybutton.popoverTargetElement = mypopover;
  mybutton.setAttribute("popovertarget", "");
  assert_equals(mybutton.popoverTargetElement, null,
    'Setting the popovertarget attribute to empty string right after setting explicit element does remove the explicit element.');

  mybutton.setAttribute("popovertarget", "mypopover");
  assert_equals(mybutton.popoverTargetElement.id, "mypopover",
    'Setting the popovertarget attribute to a value should set the popover target element.');
  mybutton.setAttribute("popovertarget", "");
  assert_equals(mybutton.getAttribute('popovertarget'), "",
    'Assigning to element.popoverTargetElement to empty string should update the attribute value to empty string.');
  assert_equals(mybutton.popoverTargetElement, null,
    'Setting the popovertarget attribute to empty string should remove the existing element from element.popoverTargetElement.');
}, 'Element attribute reflection of popoverTargetElement/popovertarget should be kept in sync.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/popovers/popovertarget-reflection.html"
}
```
