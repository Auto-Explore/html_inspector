# html/semantics/embedded-content/the-object-element/block-object-with-ruby-crash.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-object-element/block-object-with-ruby-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Morten Stenshorne" href="mailto:mstensho@chromium.org">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=966363">
<object style="display:block;">
  <ruby></ruby>
</object>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
  test(()=> {}, "no crash");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 215,
        "byte_start": 184,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.ruby.missing.rp_rt",
      "message": "Element “ruby” is missing a required instance of one or more of the following child elements: “rp”, “rt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 224,
        "byte_start": 218,
        "col": 3,
        "line": 5
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
  "source_name": "html/semantics/embedded-content/the-object-element/block-object-with-ruby-crash.html"
}
```
