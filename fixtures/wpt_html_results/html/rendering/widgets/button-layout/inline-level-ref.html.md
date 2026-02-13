# html/rendering/widgets/button-layout/inline-level-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/button-layout/inline-level-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reference for button with inline-level display</title>
<style>
button { font: inherit }
</style>
<p>There should be three buttons below containing "1" and "2" on separate lines, and "a" and "b" before and after on the same baseline as the "2".</p>
<p>a<button>1<br>2</button>b</p>
<p>a<button>1<br>2</button>b</p>
<p>a<button>1<br>2</button>b</p>
<p>a<button>1<br>2</button>b</p>
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
  "source_name": "html/rendering/widgets/button-layout/inline-level-ref.html"
}
```
