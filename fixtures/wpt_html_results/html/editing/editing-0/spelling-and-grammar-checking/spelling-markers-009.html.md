# html/editing/editing-0/spelling-and-grammar-checking/spelling-markers-009.html

Counts:
- errors: 0
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/editing-0/spelling-and-grammar-checking/spelling-markers-009.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html lang="en">
<meta charset="utf-8">
<title>Turning off spellcheck by making input elements readonly</title>
<link rel=match href="references/spelling-markers-001-ref.html">
<link rel=help href="https://html.spec.whatwg.org/multipage/interaction.html#spelling-and-grammar-checking">
<meta name=assert content="Spellchecking stops applying to input elements when they become readonly">

<style>
#test {
  /* Match the ref */
  all: initial;
  width: 100%;
  display: block;
  font-family: inherit;
}
</style>

<input type=text id=test value="This test passes if there is no visual marker indicating the spellinnnnnggg mistake in this sentence, and fails otherwise.">

<script>
  var test = document.getElementById("test");
  // Force spellcheck by focus then blur
  test.focus();
  test.blur();
  test.setAttribute("readonly", true);
</script>
```

```json
{
  "messages": [],
  "source_name": "html/editing/editing-0/spelling-and-grammar-checking/spelling-markers-009.html"
}
```
