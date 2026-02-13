# html/semantics/embedded-content/the-iframe-element/srcdoc-attribute-reset.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/srcdoc-attribute-reset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>Verify that clearing srcdoc resets the iframe's content.</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/iframe-embed-object.html#process-the-iframe-attributes">
<link rel="author" title="James MacLean" href="mailto:wjmaclean@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe id=myframe srcdoc='srcdoc_text'></iframe>
<script>
  'use strict';

  async_test(function(t) {
    window.onload = () => {
      // Verify that the srcdoc content is loaded before we start.
      t.step(() => {
        assert_true(typeof myframe.contentDocument !== 'undefined',
            'iframe has contentDocument');
        assert_equals(myframe.contentDocument.body.innerText, 'srcdoc_text',
            'iframe contains srcdoc content');
      });

      myframe.onload = t.step_func_done(function() {
        assert_true(typeof myframe.contentDocument !== 'undefined',
            'iframe has contentDocument');
        assert_equals(myframe.contentDocument.body.innerText, '',
            'iframe content is empty');
      });

      // Don't remove srcdoc until the initial load has completed, and the
      // frame's onload handler is in place.
      myframe.removeAttribute('srcdoc');
    };
  }, 'Verify that the frame reloads with empty body after we remove srcdoc.');
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/srcdoc-attribute-reset.html"
}
```
