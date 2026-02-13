# html/semantics/forms/the-button-element/button-submit-children.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-button-element/button-submit-children.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" title="Joey Arhar" href="mailto:jarhar@chromium.org">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe name=frame1 id=frame1></iframe>
<form id=form1 target=frame1 action="does_not_exist.html">
  <div id=parentdiv>
    <button id=submitbutton type=submit>
      <div id=buttonchilddiv>
        button child div text
      </div>
    </button>
  </div>
</form>

<script>
async_test(t => {
  window.addEventListener('load', () => {
    const frame1 = document.getElementById('frame1');
    frame1.addEventListener('load', t.step_func_done(() => {}));

    const submitButton = document.getElementById('submitbutton');
    submitButton.addEventListener('click', event => {
      event.preventDefault();
      const form = document.getElementById('form1');
      form.submit();
    });

    const buttonChildDiv = document.getElementById('buttonchilddiv');
    buttonChildDiv.click();
  });
}, 'This test will pass if a form navigation successfully occurs when clicking a child element of a <button type=submit> element with a onclick event handler which prevents the default form submission and manually calls form.submit() instead.');

async_test(t => {
  window.addEventListener('load', () => {
    const frame1 = document.getElementById('frame1');
    frame1.addEventListener('load', t.step_func_done(() => {}));

    const submitButton = document.getElementById('submitbutton');
    submitButton.addEventListener('click', event => {
      const form = document.getElementById('form1');
      form.submit();
    });

    const parentDiv = document.getElementById("parentdiv");
    parentDiv.addEventListener("click", event => {
      // event was already handled for the button
      // but it's activation behavior won't have run yet and we prevent that now
      event.preventDefault();
    })

    submitButton.click();
  });
}, "clicking a submit button, which calls form.submit and has a parent calling e.prevenDefault() should still submit the form");
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
        "byte_end": 410,
        "byte_start": 387,
        "col": 7,
        "line": 12
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
  "source_name": "html/semantics/forms/the-button-element/button-submit-children.html"
}
```
