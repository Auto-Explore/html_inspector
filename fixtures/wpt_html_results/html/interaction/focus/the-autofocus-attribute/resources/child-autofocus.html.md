# html/interaction/focus/the-autofocus-attribute/resources/child-autofocus.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/resources/child-autofocus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<input id="target" value="This should be unfocused!" autofocus></input>

<script>
  let got_focus = false;
  document.getElementById("target").addEventListener("focus", () => {
    got_focus = true;
  });

  window.addEventListener("load", () => {
    parent.postMessage("child_loaded", "*");
  });

  window.addEventListener("message", event => {
    if (event.data == "report_focus_state") {
      let msg = got_focus ? "child_is_focused" : "child_is_not_focused";
      parent.postMessage(msg, "*");
    }
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 87,
        "byte_start": 79,
        "col": 64,
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/resources/child-autofocus.html"
}
```
