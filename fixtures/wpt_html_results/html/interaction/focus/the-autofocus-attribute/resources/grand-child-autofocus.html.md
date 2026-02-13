# html/interaction/focus/the-autofocus-attribute/resources/grand-child-autofocus.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/the-autofocus-attribute/resources/grand-child-autofocus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<input id="target" value="This should be focused!" autofocus></input>

<script>
  let got_focus = false;
  target.addEventListener("focus", () => got_focus = true);

  window.addEventListener("load", () => {
    parent.postMessage("grand_child_loaded", "*");
  });

  window.addEventListener("message", event => {
    if (event.data == "report_focus_state") {
      let msg = got_focus ? "grand_child_is_focused" : "grand_child_is_not_focused";
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
        "byte_end": 85,
        "byte_start": 77,
        "col": 62,
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
  "source_name": "html/interaction/focus/the-autofocus-attribute/resources/grand-child-autofocus.html"
}
```
