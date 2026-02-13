# html/semantics/popovers/popover-open-overflow-display.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-open-overflow-display.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=author href="mailto:masonf@chromium.org">
<link rel=help href="https://open-ui.org/components/popover.research.explainer">
<link rel=help href="https://html.spec.whatwg.org/multipage/popover.html">
<link rel=match href="popover-open-overflow-display-ref.html">
<meta name="fuzzy" content="maxDifference=0-1;totalPixels=0-2">

<div id=container>
  <div popover id=p1>This is popover 1<div id=anchor2></div></div>
  <div popover id=p2 anchor=anchor2>This is popover 2<div id=anchor3></div></div>
  <div popover id=p3 anchor=anchor3>This is popover 3</div>
</div>

<style>
  #container {
    overflow:hidden;
    position: absolute;
    top: 100px;
    left: 50px;
    width: 30px;
    height: 30px;
  }
  #p2 {
    position: absolute;
    top: 100px;
  }
  #p3 {
    position: relative;
    top:200px;
  }
</style>

<script>
  document.querySelector('#p1').showPopover();
  document.querySelector('#p2').showPopover();
  document.querySelector('#p3').showPopover();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 618,
        "byte_start": 611,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/popovers/popover-open-overflow-display.tentative.html"
}
```
