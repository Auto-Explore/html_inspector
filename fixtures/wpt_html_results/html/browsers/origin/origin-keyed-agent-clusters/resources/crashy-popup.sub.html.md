# html/browsers/origin/origin-keyed-agent-clusters/resources/crashy-popup.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/origin-keyed-agent-clusters/resources/crashy-popup.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>This page helps exhibit a crash bug when window.open()ed (see ../popups-crash.https.html)</title>

<iframe src="https://{{hosts[][www]}}:{{ports[https][0]}}/html/browsers/origin/origin-keyed-agent-clusters/resources/send-oac-header.py"></iframe>
<iframe src="https://{{hosts[][www]}}:{{ports[https][0]}}/html/browsers/origin/origin-keyed-agent-clusters/resources/send-oac-header.py?header=?1"></iframe>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “https://{{hosts[][www]}}:{{ports[https][0]}}/html/browsers/origin/origin-keyed-agent-clusters/resources/send-oac-header.py” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 282,
        "byte_start": 145,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “https://{{hosts[][www]}}:{{ports[https][0]}}/html/browsers/origin/origin-keyed-agent-clusters/resources/send-oac-header.py?header=?1” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 439,
        "byte_start": 292,
        "col": 1,
        "line": 6
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
  "source_name": "html/browsers/origin/origin-keyed-agent-clusters/resources/crashy-popup.sub.html"
}
```
