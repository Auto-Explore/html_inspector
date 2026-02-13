# html/rendering/widgets/input-checkbox-switch.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-checkbox-switch.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<title>Checkbox with switch attribute set renders differently than a checkbox without switch attribute</title>
<link rel=match href="input-checkbox-switch-ref.html">
<link rel=mismatch href="input-checkbox-switch-notref.html">
<input type=checkbox switch>
<input id='input2' type=checkbox>
<input id='input3' type=checkbox switch>
<script>
    input2.setAttribute('switch','');
    input3.removeAttribute('switch');
    document.documentElement.classList.remove("reftest-wait");
</script>
</html>
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
  "source_name": "html/rendering/widgets/input-checkbox-switch.tentative.html"
}
```
