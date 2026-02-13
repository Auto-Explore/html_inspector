# html/semantics/interactive-elements/the-details-element/nested-details-crash.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-details-element/nested-details-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<link rel=author href="mailto:vmpstr@chromium.org">
<link rel=help href="https://crbug.com/1270206">

<script type="text/javascript">
function eventHandler1() {
  document.getElementById('target').insertAdjacentText("afterEnd", "");
  document.getElementById('target').focus();
  document.getElementById('target').hidden = "true";
}
function operate() {
  document.addEventListener('DOMNodeInsertedIntoDocument', eventHandler1, true);
}
function exec_event() {
  event = new Event('DOMNodeInsertedIntoDocument')
  document.dispatchEvent(event)
}
function go(){
  operate();
  exec_event();
}
</script>
<body onload="go();" contentEditable="true">
<details onselectstart='eventHandler2();'>
<dfn id='target' class=onload='eventHandler1();'>
<details id= onmsgesturehold='eventHandler2();'>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 149,
        "byte_start": 118,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": null
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
  "source_name": "html/semantics/interactive-elements/the-details-element/nested-details-crash.html"
}
```
