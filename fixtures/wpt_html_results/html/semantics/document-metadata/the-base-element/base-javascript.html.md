# html/semantics/document-metadata/the-base-element/base-javascript.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-base-element/base-javascript.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- Please update base-data.html together with this -->
<!DOCTYPE html>
<meta charset="utf-8">
<title>&lt;base> and javascript: URLs</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<base href="javascript:/,test">
<base href="https://example.com/">
<div id=log></div>
<script>
test(() => {
  const link = document.createElement("a");
  link.href = "blah";
  assert_equals(link.href, new URL("blah", document.URL).href);
}, "First <base> has a javascript: URL so fallback is used");

test(() => {
  document.querySelector("base").remove();
  const link = document.createElement("a");
  link.href = "blah";
  assert_equals(link.href, new URL("blah", "https://example.com/").href);
}, "First <base> is removed so second is used");

test(() => {
  const base = document.createElement("base");
  base.href = "javascript:/,more-test";
  document.head.prepend(base);
  const link = document.createElement("a");
  link.href = "blah";
  assert_equals(link.href, new URL("blah", document.URL).href);
}, "Dynamically inserted first <base> has a javascript: URL so fallback is used");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 275,
        "byte_start": 244,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 310,
        "byte_start": 276,
        "col": 1,
        "line": 8
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
  "source_name": "html/semantics/document-metadata/the-base-element/base-javascript.html"
}
```
