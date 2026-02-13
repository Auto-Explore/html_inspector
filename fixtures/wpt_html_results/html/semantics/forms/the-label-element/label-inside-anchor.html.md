# html/semantics/forms/the-label-element/label-inside-anchor.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-label-element/label-inside-anchor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>label element: clicking on label containing inline element placed inside &lt;a&gt; </title>
<link rel="author" title="Yu Han" href="mailto:yuzhehan@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/forms.html#the-label-element">
<link rel="help" href="https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-a-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<a href="javascript:void(0)" target="_blank">
    <label for="peas"><span id="text">peas?</span></label>
    <input type="checkbox" name="peas" id="peas">
</a>
<script>
  const text = document.getElementById('text'),
        peas_cb =  document.getElementById('peas');

  t1 = async_test("click on inline element inside a label that's placed inside a anchor should trigger default label behavior");

  peas_cb.onchange = t1.step_func_done(function(e) {
    assert_true(peas_cb.checked, "checkbox is checked");
  });

  t1.step(function() {
    text.click();
  });

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-label-element/label-inside-anchor.html"
}
```
