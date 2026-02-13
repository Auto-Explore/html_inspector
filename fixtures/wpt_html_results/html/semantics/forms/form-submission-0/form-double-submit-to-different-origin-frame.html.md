# html/semantics/forms/form-submission-0/form-double-submit-to-different-origin-frame.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/form-double-submit-to-different-origin-frame.html",
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
<script src="/common/get-host-info.sub.js"></script>

<!-- The onclick submit() should get superseded by the default
     action submit(), which isn't preventDefaulted by onclick here.
     This is per the Form Submission Algorithm [1], step 22.3, which
     says that new planned navigations replace old planned navigations.
    [1] https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#form-submission-algorithm
  -->

<label for=frame1 style="display:block">This frame should stay blank</label>
<iframe name=frame1 id=frame1></iframe>
<label for=frame2 style="display:block">This frame should navigate (to 404)</label>
<iframe name=frame2 id=frame2></iframe>
<form id="form1" target="frame1" action="nonexistent.html">
  <input type=hidden name=navigated value=1>
  <input id=submitbutton type=submit>
</form>

<script>
promise_test(async () => {
  function getLoadPromise(frame) {
    return new Promise(resolve => {
      frame.addEventListener('load', resolve);
    });
  }

  const frame1 = document.getElementById('frame1');
  const frame2 = document.getElementById('frame2');
  await getLoadPromise(window);

  const frame1LoadPromise = getLoadPromise(frame1);
  let frame2LoadPromise = getLoadPromise(frame2);
  const subframeUrl = get_host_info().REMOTE_ORIGIN;
  frame1.src = subframeUrl;
  frame2.src = subframeUrl;
  await frame1LoadPromise;
  await frame2LoadPromise;
  assert_false(!!frame1.contentDocument, 'frame1 should have a different origin.');
  assert_false(!!frame2.contentDocument, 'frame2 should have a different origin.');

  window.frame1Navigated = false;
  frame1.addEventListener('load', () => {
    window.frame1Navigated = true;
  });
  frame2LoadPromise = getLoadPromise(frame2);

  form1.addEventListener('click', () => {
    form1.submit();
    form1.target = 'frame2';
  });
  submitbutton.click();
  await frame2LoadPromise;

  assert_false(window.frame1Navigated, 'frame1 should not be navigated by the form submission.');
}, 'default submit action should supersede onclick submit() for cross-origin iframes');

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
        "byte_end": 801,
        "byte_start": 761,
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
        "byte_end": 918,
        "byte_start": 878,
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
  "source_name": "html/semantics/forms/form-submission-0/form-double-submit-to-different-origin-frame.html"
}
```
