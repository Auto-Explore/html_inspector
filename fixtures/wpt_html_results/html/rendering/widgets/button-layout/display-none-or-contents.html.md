# html/rendering/widgets/button-layout/display-none-or-contents.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/display-none-or-contents.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>button (in button layout) with display: none/contents</title>
<link rel=match href=display-none-or-contents-ref.html>
<link rel=help href=https://html.spec.whatwg.org/multipage/rendering.html#button-layout-2>
<style>
#none{ display: none}
#contents { display: contents; font: initial }
</style>
<!-- Button layout should not impact "display: none" or "display: contents" on button elements -->
<!-- https://github.com/whatwg/html/pull/10244 -->
<button id=none>1</button>
<button id=contents>2</button>
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
  "source_name": "html/rendering/widgets/button-layout/display-none-or-contents.html"
}
```
