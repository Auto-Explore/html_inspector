# html/semantics/forms/form-submission-0/form-data-set-usv-form.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/form-submission-0/form-data-set-usv-form.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>This is the form that will be submitted</title>

<form action="form-echo.py" method="post" enctype="text/plain">
  <input id="input1" type="text">
  <select id="input2">
    <option selected>option
  </select>
  <input id="input3" type="radio" checked>
  <input id="input4" type="checkbox" checked>
</form>

<script>
"use strict";

const form = document.querySelector("form");

for (let el of Array.from(form.querySelectorAll("input"))) { // Firefox/Edge support
  el.name = el.id + "\uDC01";
  el.value = el.id + "\uDC01";
}

const select = document.querySelector("select");
select.name = select.id + "\uDC01";
select.firstElementChild.value = select.id + "\uDC01";
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
  "source_name": "html/semantics/forms/form-submission-0/form-data-set-usv-form.html"
}
```
