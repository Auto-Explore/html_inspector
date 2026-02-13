# html/semantics/forms/the-textarea-element/textarea-select-event-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-select-event-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLTextAreaElement Test: select event</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<meta name="flags" content="interact">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<p>Select any numberic characters in the text area below</p>

<form id="testForm" name="testForm">
  <textarea id="testtextarea">0123456789</textarea>
</form>

<script>

var textarea = document.getElementById("testtextarea");

setup({explicit_done : true});
setup({explicit_timeout : true});

on_event(textarea, "select", function(evt) {
  test(function() {
    assert_greater_than(textarea.value.substring(textarea.selectionStart, textarea.selectionEnd).length, 0, "Check if select event captured when text selected");
  });
  done();
});

</script>

<div id="log"></div>
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-select-event-manual.html"
}
```
