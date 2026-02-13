# html/semantics/embedded-content/the-iframe-element/srcdoc-anchor.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/srcdoc-anchor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>Verify srcdoc content loads when src is about:srcdoc#foo.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe id=myframe srcdoc='srcdoc_text' src='about:srcdoc#foo'></iframe>

<script>
  async_test(function(t) {
    // Verify that the srcdoc content is loaded before we start.
    window.onload = t.step_func_done(() => {
        assert_true(typeof myframe.contentDocument !== 'undefined',
            'iframe has contentDocument');
        assert_equals(myframe.contentDocument.body.innerText, 'srcdoc_text',
            'iframe contains srcdoc content');
    }, '');
  }, 'Verify srcdoc content loads when src is about:srcdoc#foo.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/embedded-content/the-iframe-element/srcdoc-anchor.html"
}
```
