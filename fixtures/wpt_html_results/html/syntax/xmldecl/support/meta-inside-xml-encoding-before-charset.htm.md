# html/syntax/xmldecl/support/meta-inside-xml-encoding-before-charset.htm

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/xmldecl/support/meta-inside-xml-encoding-before-charset.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<?xml version="1.0" <meta encoding="windows-1251" charset="windows-1253"?>
<p>Meta inside XML declaration with shared greater-than; encoding before charse</p>
<p>Test: �</p>
<p>If &#x03B6;, meta takes precedence. If &#x0436;, XML decl takes precedence</p>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.processing_instruction",
      "message": "Saw “<?”. Probable cause: Attempt to use an XML processing instruction in HTML. (XML processing instructions are not supported in HTML.)",
      "severity": "Warning",
      "span": {
        "byte_end": 2,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 78,
        "byte_start": 75,
        "col": 1,
        "line": 2
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
  "source_name": "html/syntax/xmldecl/support/meta-inside-xml-encoding-before-charset.htm"
}
```
