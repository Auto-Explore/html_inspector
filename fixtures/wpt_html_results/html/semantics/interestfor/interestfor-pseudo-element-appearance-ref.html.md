# html/semantics/interestfor/interestfor-pseudo-element-appearance-ref.html

Counts:
- errors: 1
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-pseudo-element-appearance-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel="author" href="mailto:masonf@chromium.org">

<div class=example>
  <button class=invoker><span class=before>::before</span><span>Button</span><span class=after>::after</span></button>
  <button class=interesthint>::interest-hint</button>
</div>
<div class=example>
  <a href=# class=invoker><span class=before>::before</span><span>Link</span><button class=interesthint>::interest-hint</button><span class=after>::after</span></a>
</div>
<div class=example>
  <img src="/images/blue.png" usemap="#map1" width=40 height=40>
  <span class=before>::before</span><map id="map1"><area href="/" shape="default" class=invoker></map><button class=interesthint>::interest-hint</button><span class=after>::after</span>
</div>

<div id=target></div>

<style>
  .before {
    content: "::before";
    border: 1px solid green;
  }
  /* UA stylesheet for ::interest-hint: */
  .interesthint {
    user-select: none;
    color: buttontext;
    display: inline-block;
    text-align: center;
    cursor: default;
    background-color: buttonface;
    margin-inline-start: 0.5em;
    padding-block: 1px;
    padding-inline: 6px;
    border-width: 1px;
    border-radius: 5px;
    border-style: outset;
    border-color: buttonborder;
  }
  .after {
    content: "::after";
    border: 1px solid green;
  }
  /* Test convenience: */
  .example {
    display:block;
    width: 400px;
    height: 50px;
  }
  .invoker>span {
    border: 1px solid black;
  }
  .interesthint {
    font-family: arial;
    font-size: 13px;
  }
</style>

<script>
  // Fix up parser moving out the button
  const interestButton = document.querySelector('button.invoker');
  const movedButton = document.querySelector('button.invoker + button.interesthint');
  const afterSpan = interestButton.querySelector('.after');
  interestButton.insertBefore(movedButton, afterSpan);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 572,
        "byte_start": 510,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 797,
        "byte_start": 790,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map1”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 572,
        "byte_start": 510,
        "col": 3,
        "line": 13
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
  "source_name": "html/semantics/interestfor/interestfor-pseudo-element-appearance-ref.html"
}
```
