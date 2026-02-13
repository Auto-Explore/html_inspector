# html/semantics/popovers/popover-anchor-multicol-display.tentative.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/popovers/popover-anchor-multicol-display.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Tests popovers with implicit anchors in out-of-flow boxes</title>
<link rel="help" href="https://drafts.csswg.org/css-anchor-1/#determining">
<link rel="help" href="https://drafts.csswg.org/css-anchor-1/#propdef-anchor-name">
<link rel="help" href="https://drafts.csswg.org/css-anchor-1/#anchor-size">
<link rel="author" href="mailto:xiaochengh@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/check-layout-th.js"></script>

<style>
.relpos {
  position: relative;
}
.columns {
  column-count: 2;
  column-fill: auto;
  column-gap: 10px;
  column-width: 100px;
  width: 210px;
  height: 50px;
}
#anchor1 {
  position: absolute;
  width: 10px;
  height: 30px;
  background: orange;
}
.target {
  /*
   * We need a popover to use implicit anchors, and force showing it with CSS
   * so that it's not in the top layer.
   */
  display: block;
  position: absolute;
  margin: 0;
  border: 0;
  padding: 0;
  width: anchor-size(width);
  height: anchor-size(height);
  background: lime;
}
</style>
<body onload="checkLayout('.target')">
  <div class="spacer" style="height: 10px"></div>
  <div class="relpos">
    <div class="columns">
      <div class="spacer" style="height: 10px"></div>
      <div class="relpos">
        <div class="spacer" style="height: 10px"></div>
        <div class="relpos">
          <div class="spacer" style="height: 10px"></div>
          <div id="anchor1"></div>
        </div>
        <div class="target" popover anchor="anchor1" data-expected-height=30></div>
      </div>
      <div class="target" popover anchor="anchor1" data-expected-height=50></div>
    </div>
  </div>
</body>

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
  "source_name": "html/semantics/popovers/popover-anchor-multicol-display.tentative.html"
}
```
