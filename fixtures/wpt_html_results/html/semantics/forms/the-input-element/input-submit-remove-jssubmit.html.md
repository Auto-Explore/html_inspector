# html/semantics/forms/the-input-element/input-submit-remove-jssubmit.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/input-submit-remove-jssubmit.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-2">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe name="frame" id="frame"></iframe>
<form id="form" target="frame" action="does_not_exist.html">
  <input id="input" name="name" value="foo">
  <input id="submitbutton" type="submit"></input>
</form>

<script>
async_test(t => {
  window.addEventListener('load', () => {
    const frame = document.getElementById('frame');
    frame.addEventListener('load', t.step_func_done(() => {
      const expected = (new URL("does_not_exist.html?name=bar", location.href)).href;
      assert_equals(frame.contentWindow.location.href, expected);
    }));

    const form = document.getElementById('form');
    const input = document.getElementById('input');
    const submitButton = document.getElementById('submitbutton');
    submitButton.addEventListener('click', event => {
      submitButton.remove();
      form.submit();
      input.value = "bar";
      form.submit();
      input.value = "baz";
    });

    submitButton.click();
  });
}, 'This test will pass if a form navigation successfully occurs when clicking a <input type=submit> element with a onclick event handler which removes the input and then calls form.submit().');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 458,
        "byte_start": 450,
        "col": 42,
        "line": 10
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
  "source_name": "html/semantics/forms/the-input-element/input-submit-remove-jssubmit.html"
}
```
