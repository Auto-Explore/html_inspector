# html/semantics/selectors/pseudo-classes/active-disabled.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/selectors/pseudo-classes/active-disabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/7465">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<label id=buttonlabel for=disabledbutton>label for disabled button</label>
<button id=disabledbutton disabled>disabled</button>

<button id=buttonparent disabled>
  <div id=buttonchild>child of disabled</div>
</button>

<input id=disabledinput disabled>

<textarea id=disabledtextarea disabled>disabled textarea</textarea>

<script>
function testElement(description, clickElement, checkElement) {
  promise_test(async () => {
    if (!checkElement)
      checkElement = clickElement;

    await (new test_driver.Actions()
      .pointerMove(2, 2, {origin: clickElement})
      .pointerDown())
      .send();

    assert_true(checkElement.matches(':active'));

    await (new test_driver.Actions()
      .pointerUp())
      .send();
  }, description);
}

testElement('Clicking on a disabled button should make it match the :active selector.',
    disabledbutton);

testElement('Clicking the label for a disabled button should make the button match the :active selector.',
    buttonlabel, disabledbutton);

testElement('Clicking on a child of a disabled button should make the button match the :active selector.',
    buttonchild, buttonparent);

testElement('Clicking on a disabled input should make it match the :active selector.',
    disabledinput);

testElement('Clicking on a disabled textarea should make it match the :active selector.',
    disabledtextarea);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.child.disallowed_in_phrasing_parent",
      "message": "Element “div” not allowed as child of “button” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 586,
        "byte_start": 566,
        "col": 3,
        "line": 14
      }
    },
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
  "source_name": "html/semantics/selectors/pseudo-classes/active-disabled.html"
}
```
