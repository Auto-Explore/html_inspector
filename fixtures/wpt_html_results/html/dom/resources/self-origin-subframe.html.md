# html/dom/resources/self-origin-subframe.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/resources/self-origin-subframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script>
  window.onmessage = function(e){
    if (e.data == "getOrigin") {
      parent.postMessage(self.origin, "*");
    } else if (e.data == "setDomainAndGetOrigin") {
      var oldDomain = document.domain;
      try {
        document.domain = document.domain.replace(/^[^.]*./, "");
      } catch (e) {
        parent.postMessage("THREW WHEN SETTING DOMAIN: " + e, "*");
        return;
      }
      if (oldDomain === document.domain) {
        parent.postMessage("FAILED TO SET DOMAIN", "*");
      } else {
        parent.postMessage(self.origin, "*");
      }
    } else {
      parent.postMessage("UNEXPECTED MESSAGE", "*");
    }
  }
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
  "source_name": "html/dom/resources/self-origin-subframe.html"
}
```
