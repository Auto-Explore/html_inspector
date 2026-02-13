# html/webappapis/user-prompts/resources/print-during-event.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/user-prompts/resources/print-during-event.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Page that tries to print during an event</title>

<script>
"use strict";

window.on{{GET[event]}} = () => {
  try {
    window.print();
  } catch (e) {
    window.opener.postMessage(`error: ${e.message}`);
  }
};

window.opener.postMessage("start", "*");
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
  "source_name": "html/webappapis/user-prompts/resources/print-during-event.sub.html"
}
```
