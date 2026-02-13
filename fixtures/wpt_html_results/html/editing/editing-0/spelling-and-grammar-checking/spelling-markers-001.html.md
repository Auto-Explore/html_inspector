# html/editing/editing-0/spelling-and-grammar-checking/spelling-markers-001.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/spelling-and-grammar-checking/spelling-markers-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<meta charset="utf-8">
<title>Turning off spellcheck on editing hosts</title>
<link rel=match href="references/spelling-markers-001-ref.html">
<link rel=help href="https://html.spec.whatwg.org/multipage/interaction.html#spelling-and-grammar-checking">
<meta name=assert content="Spellchecking stops applying to editing hosts when they become non editable">

<div id="test" contenteditable=true>This test passes if there is no visual marker indicating the spellinnnnnggg mistake in this sentence, and fails otherwise.</div>

<script>
  var test = document.getElementById("test");
  // Force spellcheck by focus and then blur
  test.focus();
  test.blur();
  test.removeAttribute("contenteditable");
</script>
```

```json
{
  "messages": [],
  "source_name": "html/editing/editing-0/spelling-and-grammar-checking/spelling-markers-001.html"
}
```
