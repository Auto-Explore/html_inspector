# html/semantics/popovers/popover-anchor-display-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-display-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<link rel=author href="mailto:masonf@chromium.org">

<p>There should be a green box attached to the right side of each orange box.</p>
<div id=proper_anchors class=wrapper>
  <div><div class=anchor></div><div class=popover></div></div>
  <div><div class=anchor></div><div class=popover></div></div>
  <div><div class=anchor></div><div class=popover></div></div>
  <div><div class=anchor></div><div class=popover></div></div>
  <div><div class=anchor></div><div class=popover></div></div>
  <div><div class=anchor></div><div class=popover></div></div>
</div>

<p>There should NOT be any red boxes next to the orange ones. (Red ones down below are ok.)</p>
<div id=improper_anchors class=wrapper>
  <div><div class=anchor></div><div class=popover></div></div>
  <div><div class=anchor></div><div class=popover></div></div>
</div>

<style>
  body { margin:0; padding:0; box-sizing: border-box; }
  p {height: 25px;}
  .wrapper {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }
  .wrapper>div {
    position:relative;
    width: 125px;
    height: 75px;
    font-size: 0;
  }
  .wrapper>div div {
    display:inline-block;
    width: 50px;
    height: 50px;
  }
  .anchor {
    background: orange;
  }
  #proper_anchors .popover {
    background: lime;
  }
  #improper_anchors .popover {
    background: red;
    position:fixed;
    left: 0;
    top: 0;
  }
</style>
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
        "byte_end": 875,
        "byte_start": 868,
        "col": 1,
        "line": 21
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
  "source_name": "html/semantics/popovers/popover-anchor-display-ref.html"
}
```
