# html/browsers/sandboxing/resources/sandbox-inherited-from-initiator-response-helper.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/resources/sandbox-inherited-from-initiator-response-helper.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script>
  const iframe_1_script = encodeURI(`
    <script>
      try {
        document.domain = document.domain;
        parent.postMessage("not sandboxed", "*");
      } catch (exception) {
        parent.postMessage("sandboxed", "*");
      }
    </scr`+`ipt>
  `);

  const iframe_1 = parent.document.querySelector("#iframe_1");
  iframe_1.src = `data:text/html,${iframe_1_script}`;
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
        "byte_end": 8,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/browsers/sandboxing/resources/sandbox-inherited-from-initiator-response-helper.html"
}
```
