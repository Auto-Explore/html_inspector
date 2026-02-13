# html/browsers/origin/inheritance/resources/document-write.html

Counts:
- errors: 3
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/inheritance/resources/document-write.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <script src="/resources/testharness.js"></script>
    <script src="/common/get-host-info.sub.js"></script>
  </head>
  <body></body>
  <script>
    const domain_start = document.domain;
    const domain_new = domain_start.replace(/^[^.]+\./,'');

    async_test(test => {
      const iframe = document.createElement('iframe');
      iframe.src = './iframe-with-about-blank-iframe.html';
      iframe.onload = test.step_func_done(() => {
        const doc0 = frames[0].frames[0].document;
        const doc1 = frames[0].frames[1].document;

        assert_equals(doc0.domain, domain_start);
        assert_equals(doc1.domain, domain_start);

        doc0.open();
        doc1.open();
        assert_equals(doc0.domain, domain_start);
        assert_equals(doc1.domain, domain_start);

        document.domain = domain_new;
        assert_equals(doc0.domain, domain_start);
        assert_equals(doc1.domain, domain_start);

        doc0.close();
        doc1.close();
        assert_equals(doc0.domain, domain_start);
        assert_equals(doc1.domain, domain_start);
      });
      document.body.appendChild(iframe);
    }, "document.open() do not make the callee's origin to alias the caller's"
      + " one");
  </script>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 179,
        "byte_start": 171,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1252,
        "byte_start": 179,
        "col": 11,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1261,
        "byte_start": 1252,
        "col": 3,
        "line": 39
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
  "source_name": "html/browsers/origin/inheritance/resources/document-write.html"
}
```
