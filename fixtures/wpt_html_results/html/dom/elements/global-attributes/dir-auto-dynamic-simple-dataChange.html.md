# html/dom/elements/global-attributes/dir-auto-dynamic-simple-dataChange.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dir-auto-dynamic-simple-dataChange.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class="reftest-wait">
<meta charset="utf-8">
<title>Dynamic changes with textContent and dir=auto</title>
<link rel="match" href="dir-auto-dynamic-simple-ref.html">
<div>Test for elements with dir="auto" whose content changes between directional and neutral</div>
<div dir="auto" id="from_ltr_to_ltr">abc</div>
<div dir="auto" id="from_ltr_to_rtl">abc</div>
<div dir="auto" id="from_ltr_to_neutral">abc</div>
<div dir="auto" id="from_rtl_to_ltr">אבג</div>
<div dir="auto" id="from_rtl_to_rtl">אבג</div>
<div dir="auto" id="from_rtl_to_neutral">אבג</div>
<div dir="auto" id="from_neutral_to_ltr">123</div>
<div dir="auto" id="from_neutral_to_rtl">123</div>
<div dir="auto" id="from_neutral_to_neutral">123</div>
<script>
function changeContent() {
  var directionalTexts = {ltr:"xyz", rtl:"ابج", neutral:"456"};

  for (var dirFrom in directionalTexts) {
    for (var dirTo in directionalTexts) {
      var element = document.getElementById("from_" + dirFrom +
                                            "_to_" + dirTo);
      element.firstChild.data = directionalTexts[dirTo];
    }
  }
  document.documentElement.removeAttribute("class");
}

document.addEventListener("TestRendered", changeContent);
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
  "source_name": "html/dom/elements/global-attributes/dir-auto-dynamic-simple-dataChange.html"
}
```
