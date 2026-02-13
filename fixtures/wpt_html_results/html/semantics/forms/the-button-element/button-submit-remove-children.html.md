# html/semantics/forms/the-button-element/button-submit-remove-children.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-submit-remove-children.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-2">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe name=frame1 id=frame1></iframe>
<form id=form1 target=frame1 action="does_not_exist.html">
  <button id=submitbutton type=submit>
    <span id=outerchild>
      <span id=innerchild>submit</span>
    </span>
  </button>
</form>

<script>
async_test(t => {
  window.addEventListener('load', () => {
    const frame1 = document.getElementById('frame1');
    frame1.addEventListener('load', t.step_func_done(() => {}));

    const submitButton = document.getElementById('submitbutton');
    submitButton.addEventListener('click', event => {
      document.getElementById('outerchild').remove();
    });

    document.getElementById('innerchild').click();
  });
}, 'This test will pass if a form navigation successfully occurs when clicking a child element of a <button type=submit> element with a onclick event handler which removes the button\'s child.');
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
  "source_name": "html/semantics/forms/the-button-element/button-submit-remove-children.html"
}
```
