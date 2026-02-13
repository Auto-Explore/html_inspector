# html/semantics/forms/form-submission-0/form-double-submit-3.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/form-double-submit-3.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel="author" href="mailto:masonf@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-algorithm">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- <button> should have the same double-submit protection that
     <input type=submit> has.
  -->




<label for=frame1 style="display:block">This frame should stay blank</label>
<iframe name=frame1 id=frame1></iframe>
<label for=frame2 style="display:block">This frame should navigate (to 404)</label>
<iframe name=frame2 id=frame2></iframe>
<form id="form1" target="frame1" action="nonexistent.html">
  <input type=hidden name=navigated value=1>
  <button id=submitbutton>submit</button>
</form>

<script>
let frame1 = document.getElementById('frame1');
let frame2 = document.getElementById('frame2');
let form1 = document.getElementById('form1');
let submitbutton = document.getElementById('submitbutton');

async_test(t => {
  window.addEventListener('load', () => {
    frame1.addEventListener('load', t.step_func_done(() => {
      assert_unreached("Frame1 should not get navigated by this test.");
    }));
    frame2.addEventListener('load', t.step_func_done(() => {
      let params = (new URL(frame2.contentWindow.location)).searchParams;
      let wasNavigated = !!params.get("navigated");
      assert_true(wasNavigated)
    }));
    form1.addEventListener('click', t.step_func(() => {
      form1.submit();
      form1.target='frame2';

    }));
    submitbutton.click();
  });
}, '<button> should have the same double-submit protection as <input type=submit>');

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.label.for.must_reference_non_hidden_control",
      "message": "The value of the “for” attribute of the “label” element must be the ID of a non-hidden form control.",
      "severity": "Error",
      "span": {
        "byte_end": 469,
        "byte_start": 429,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.label.for.must_reference_non_hidden_control",
      "message": "The value of the “for” attribute of the “label” element must be the ID of a non-hidden form control.",
      "severity": "Error",
      "span": {
        "byte_end": 586,
        "byte_start": 546,
        "col": 1,
        "line": 18
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
  "source_name": "html/semantics/forms/form-submission-0/form-double-submit-3.html"
}
```
