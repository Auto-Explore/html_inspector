# html/capability-delegation/resources/delegate-fullscreen-request-recipient.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/capability-delegation/resources/delegate-fullscreen-request-recipient.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Capability Delegation of Fullscreen Requests test recipient</title>
<body>Capability Delegation of Fullscreen Requests test recipient body</body>

<script>
  const initiator = window.opener ? window.opener : window.top;
  initiator.postMessage({"type": "recipient-loaded"}, "*");

  function reportResult(msg) {
      initiator.postMessage({"type": "result", "result": msg}, "*");
  }

  document.addEventListener('fullscreenchange', async () => {
      if (document.fullscreenElement) {
          await document.exitFullscreen();
          reportResult("success");
      }
  });

  document.addEventListener('fullscreenerror', () => {
      reportResult("failure");
  });

  window.addEventListener("message", e => {
      if (e.data.type == "make-fullscreen-request") {
          document.body.requestFullscreen();
      }
  });
</script>
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
        "byte_end": 178,
        "byte_start": 170,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 854,
        "byte_start": 178,
        "col": 9,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 863,
        "byte_start": 854,
        "col": 1,
        "line": 29
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
  "source_name": "html/capability-delegation/resources/delegate-fullscreen-request-recipient.html"
}
```
