# html/editing/editing-0/contenteditable/selection-in-contentEditable-at-turning-designMode-on-off.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/contenteditable/selection-in-contentEditable-at-turning-designMode-on-off.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>selection in contenteditable should not be changed when designMode is turned on/off</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<iframe srcdoc="<body contenteditable>abc</body>"></iframe>
<script>
  const test_load = async_test("Selection in contenteditable shouldn't be reinitialized when changing designMode");
  window.addEventListener("load", test_load.step_func_done(() => {
    let iframe = document.querySelector("iframe");
    let iframeSelection = iframe.contentDocument.getSelection();
    iframe.focus();
    iframeSelection.collapse(iframe.contentDocument.body, 1);
    function summariseRange(range) {
      if (!range) {
        return "null";
      }
      return `(${range.startContainer.nodeName}, ${range.startOffset}) - (${range.endContainer.nodeName}, ${range.endOffset})`;
    }
    let maybeNormalizedRangeSummary = summariseRange(iframeSelection.getRangeAt(0));
    assert_in_array(maybeNormalizedRangeSummary, ["(BODY, 1) - (BODY, 1)", "(#text, 3) - (#text, 3)"],
                    "Selection collapsed at end of <body> can be either as-is or normalized to the end of the text node");
    iframe.contentDocument.designMode = "on";
    assert_equals(summariseRange(iframeSelection.getRangeAt(0)), maybeNormalizedRangeSummary,
                  "Turning designMode on at load event shouldn't change selection in contenteditable");
    iframe.contentDocument.designMode = "off";
    assert_equals(summariseRange(iframeSelection.getRangeAt(0)), maybeNormalizedRangeSummary,
                  "Turning designMode off at load event shouldn't change selection in contenteditable");
  }));
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
  "source_name": "html/editing/editing-0/contenteditable/selection-in-contentEditable-at-turning-designMode-on-off.tentative.html"
}
```
