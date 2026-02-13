# html/the-xhtml-syntax/parsing-xhtml-documents/support/xhtml-mathml-dtd-entity.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/the-xhtml-syntax/parsing-xhtml-documents/support/xhtml-mathml-dtd-entity.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<script>
  var parser = new DOMParser();
  var parse = parser.parseFromString.bind(parser);

  function generateTestFunction(entitystring, expectedString, publicId, systemId, mimeType, friendlyMime) {
    return function () {
      var doctypeString = '<!DOCTYPE html';
      if (publicId != null)
        doctypeString += ' PUBLIC "' + publicId + '" "' + systemId + '">';
      else if (systemId != null)
        doctypeString += ' SYSTEM "' + systemId + '">';
      else // both are null
        doctypeString += '>';
      var doc = parse(doctypeString + "<html><head></head><body id='test'>"+entitystring+"</body></html>", mimeType);
      var root = doc.getElementById('test');
      parent.assert_not_equals(root, null, friendlyMime + " parsing the entity reference caused a parse error;");
      parent.assert_true(!!root.firstChild);
      // Next line because some browsers include the partial parsed result in the parser error returned document.
      parent.assert_equals(root.firstChild.nodeType, 3/*Text*/, friendlyMime + " parsing the entity reference caused a parse error;");
      var text = root.firstChild.data;
      for (var i = 0, len = expectedString.length; i < len; i++) {
        parent.assert_equals(text.charCodeAt(i),expectedString.charCodeAt(i));
      }
    }
  }

  function setupTests(jsonEntities, publicId, systemId, mimeType, friendlyMime) {
    for (entityName in jsonEntities) {
      if ((mimeType == "text/html") || /;$/.test(entityName)) {
        parent.test(generateTestFunction(entityName, jsonEntities[entityName].characters, publicId, systemId, mimeType, friendlyMime), friendlyMime + " parsing " + entityName);
      }
    }
  }

  parent.setup(function() {}, {explicit_done: true});

  function run(row) {
    var xhr = new XMLHttpRequest();
    xhr.open("GET", "entities.json");
    xhr.onload = function () {
      var entitiesJSON = JSON.parse(xhr.response);
      setupTests(entitiesJSON, row[1], row[2], row[0], row[3]);
      parent.done();
    }
    xhr.send();
  }
</script>
```

```json
{
  "messages": [
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
  "source_name": "html/the-xhtml-syntax/parsing-xhtml-documents/support/xhtml-mathml-dtd-entity.htm"
}
```
