# html/syntax/parsing/named-character-references.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/named-character-references.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Tests for known named character references</title>
<meta name=viewport content="width=device-width">
<!-- Alternative output: http://mathias.html5.org/tests/html/named-character-references/ -->
<div id=log></div>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script src=named-character-references-data.js></script>
<script>
  (function() {

    function pad(string, totalCharacters) {
      return totalCharacters < string.length ? string : (Array(totalCharacters + 1).join('0') + string).slice(-totalCharacters);
    }

    var dummy = document.createElement('p');

    Object.keys(data).forEach(function(entity) {
      var object = data[entity];
      dummy.innerHTML = entity;
      test(
        function() {
          assert_equals(
            dummy.textContent,
            object.characters
          );
        },
        entity + ' should match ' + object.codepoints.map(function(codePoint) {
          return 'U+' + pad(codePoint.toString(16).toUpperCase(), 5);
        }).join(' ')
      );
    });

  }());
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
  "source_name": "html/syntax/parsing/named-character-references.html"
}
```
