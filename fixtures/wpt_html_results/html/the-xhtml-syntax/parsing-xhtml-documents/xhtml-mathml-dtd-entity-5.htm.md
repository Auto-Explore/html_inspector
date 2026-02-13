# html/the-xhtml-syntax/parsing-xhtml-documents/xhtml-mathml-dtd-entity-5.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/the-xhtml-syntax/parsing-xhtml-documents/xhtml-mathml-dtd-entity-5.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name=timeout content=long>
<title>HTML entities for various XHTML Doctype</title>
<link rel=help href="http://w3c.github.io/html/xhtml.html#parsing-xhtml-documents">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe id="test" src="support/xhtml-mathml-dtd-entity.htm"></iframe>

<script>
onload = () => document.getElementById("test").contentWindow.run(
["application/xhtml+xml", "-//W3C//DTD XHTML Basic 1.0//EN", "foo", "XHTML Basic"]);
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
  "source_name": "html/the-xhtml-syntax/parsing-xhtml-documents/xhtml-mathml-dtd-entity-5.htm"
}
```
