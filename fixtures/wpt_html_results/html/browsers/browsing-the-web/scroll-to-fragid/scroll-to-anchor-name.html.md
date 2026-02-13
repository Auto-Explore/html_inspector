# html/browsers/browsing-the-web/scroll-to-fragid/scroll-to-anchor-name.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-to-anchor-name.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Fragment Navigation: scroll to anchor name is lower priority than equal id</title>
<meta name=timeout content=long>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<div></div>
<a name="anchor1" style="position:absolute; top:200px;"></a>
<div id="id-equals-anchor" style="position:absolute; top:300px;"></div>
<a name="id-equals-anchor" style="position:absolute; top:400px;"></a>
<a name="§1" style="position:absolute; top:400px;"></a>
<div style="height:200em;"></div>
<script>
var steps = [{
    fragid:'anchor1',
      handler: function(){
        assert_equals( scrollPosition(), 200 );
      }
    },{
      fragid:'id-equals-anchor',
      handler: function(){
        // id still takes precedence over anchor name
        assert_equals( scrollPosition(), 300 );
      }
    },{
      fragid:'§1',
      handler: function(){
        assert_equals( scrollPosition(), 400 );
      }
    }];

function scrollPosition(){
  return document.documentElement.scrollTop || document.body.scrollTop;
}

function runNextStep(){
    if( steps.length > 0 ) {
      var step = steps.shift();
      var listener = t.step_func( function(){
        step.handler();
        runNextStep();
      });
      scrollToFragmentThenDo( step.fragid, listener );
    } else {
      t.done();
    }
}

function scrollToFragmentThenDo( fragid, then ){
  location.hash = fragid;
  setTimeout( then, 1 );
}

var t = async_test();
t.step( function(){
  assert_equals(location.hash, "", "Page must be loaded with no hash");
  runNextStep();
})
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 320,
        "byte_start": 264,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 462,
        "byte_start": 397,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.a.name.obsolete",
      "message": "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 519,
        "byte_start": 467,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/scroll-to-anchor-name.html"
}
```
