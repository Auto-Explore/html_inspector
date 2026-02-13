# html/editing/dnd/events/events-suite-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/events-suite-manual.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - event sequence for draggable elements</title>
<script type="text/javascript" src="/resources/testharness.js"></script>
<script type="text/javascript" src="/resources/testharnessreport.js"></script>
<style type="text/css">
  /* use margins instead of padding to make sure the body begins at the top of the page */
  html, body {
    margin: 0;
  }
  body {
    padding: 116px 8px 8px;
  }
  #testhere div {
    height: 100px;
    width: 100px;
    position: absolute;
    top: 8px;
  }
  #orange {
    background-color: orange;
    left: 8px;
  }
  #fuchsia {
    background-color: fuchsia;
    left: 158px;
  }
  #yellow {
    background-color: yellow;
    left: 308px;
  }
  #blue {
    background-color: navy;
    left: 458px;
  }
</style>

<script>
setup(function () {},{explicit_done:true,explicit_timeout:true});
window.onload = function () {
  var orange = document.querySelector('#orange')
  var fuchsia = document.querySelector('#fuchsia')
  var yellow = document.querySelector('#yellow')
  var blue = document.querySelector('#blue')
  var body = document.body;

  var events = new Array

  orange.ondragstart = function (e) {
    events.push('orange.ondragstart');
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('Text', 'foo');
  };
  orange.ondrag = function () { events.push('orange.ondrag'); };
  orange.ondragenter = function () { events.push('orange.ondragenter'); };
  orange.ondragover = function () { events.push('orange.ondragover'); };
  orange.ondragleave = function () { events.push('orange.ondragleave'); };
  orange.ondrop = function () { events.push('orange.ondrop'); return false; };
  orange.ondragend = function () { events.push('orange.ondragend'); };
  orange.onmousedown = function () { events.push('orange.onmousedown'); };
  orange.onmouseup = function () { events.push('orange.onmouseup'); };

  /* Events for the fuchsia box */
  fuchsia.ondragstart = function () { events.push('pink.ondragstart'); };
  fuchsia.ondrag = function () { events.push('pink.ondrag'); };
  fuchsia.ondragenter = function () { events.push('pink.ondragenter'); };
  fuchsia.ondragover = function () { events.push('pink.ondragover'); };
  fuchsia.ondragleave = function () { events.push('pink.ondragleave'); };
  fuchsia.ondrop = function () { events.push('pink.ondrop'); return false; };
  fuchsia.ondragend = function () { events.push('pink.ondragend'); };
  fuchsia.onmousedown = function () { events.push('pink.onmousedown'); };
  fuchsia.onmouseup = function () { events.push('pink.onmouseup'); };

  /* Events for the fuchsia box */
  yellow.ondragstart = function () { events.push('yellow.ondragstart'); };
  yellow.ondrag = function () { events.push('yellow.ondrag'); };
  yellow.ondragenter = function () { events.push('yellow.ondragenter'); return false; };
  yellow.ondragover = function () { events.push('yellow.ondragover'); return false; };
  yellow.ondragleave = function () { events.push('yellow.ondragleave'); };
  yellow.ondrop = function () { events.push('yellow.ondrop'); return false; };
  yellow.ondragend = function () { events.push('yellow.ondragend'); };
  yellow.onmousedown = function () { events.push('yellow.onmousedown'); };
  yellow.onmouseup = function () { events.push('yellow.onmouseup'); };

  /* Events for the blue box (droppable) */
  blue.ondragstart = function () { events.push('blue.ondragstart'); };
  blue.ondrag = function () { events.push('blue.ondrag'); };
  blue.ondragenter = function () { events.push('blue.ondragenter'); return false; };
  blue.ondragover = function () { events.push('blue.ondragover'); return false; };
  blue.ondragleave = function () { events.push('blue.ondragleave'); };
  blue.ondrop = function () { events.push('blue.ondrop'); return false; };
  blue.ondragend = function () { events.push('blue.ondragend'); };
  blue.onmousedown = function () { events.push('blue.onmousedown'); };
  blue.onmouseup = function () { events.push('blue.onmouseup'); };

  /* Events for the page body */
  body.ondragstart = function (e) { events.push( ( e.target == body ) ? 'body.ondragstart': 'bubble.ondragstart' ); };
  body.ondrag = function (e) { events.push( ( e.target == body ) ? 'body.ondrag': 'bubble.ondrag' ); };
  body.ondragenter = function (e) { events.push( ( e.target == body ) ? 'body.ondragenter': 'bubble.ondragenter' ); };
  body.ondragover = function (e) { events.push( ( e.target == body ) ? 'body.ondragover': 'bubble.ondragover' ); };
  body.ondragleave = function (e) { events.push( ( e.target == body ) ? 'body.ondragleave': 'bubble.ondragleave' ); };
  body.ondrop = function (e) { events.push( ( e.target == body ) ? 'body.ondrop': 'bubble.ondrop' ); };
  body.ondragend = function (e) { events.push( ( e.target == body ) ? 'body.ondragend': 'bubble.ondragend' ); setTimeout(finish,100); };
  body.onmousedown = function (e) { events.push( ( e.target == body ) ? 'body.onmousedown': 'bubble.onmousedown' ); };
  body.onmouseup = function (e) { events.push( ( e.target == body ) ? 'body.onmouseup': 'bubble.onmouseup' ); };

  function finish(e) {
    var i, evindex;
    events = events.join('-');
    /*
      Normalise; reduce repeating event sequences to only 2 occurrences.
      This makes the final event sequence predictable, no matter how many times the drag->dragover sequences repeat.
      Two occurrances are kept in each case to allow testing to make sure the sequence really is repeating.
    */
    //spec compliant - div dragenter is not cancelled, so body dragenter fires and body becomes current target
    //repeats while drag is over orange or fuchsia or the body
    events = events.replace(/(-orange\.ondrag-bubble\.ondrag-body\.ondragover){3,}/g,'$1$1');
    //repeats while dragging over yellow
    events = events.replace(/(-orange\.ondrag-bubble\.ondrag-yellow\.ondragover-bubble\.ondragover){3,}/g,'$1$1');
    //repeats while dragging over blue
    events = events.replace(/(-orange\.ondrag-bubble\.ondrag-blue\.ondragover-bubble\.ondragover){3,}/g,'$1$1');
    //non-spec-compliant repeats while dragging over orange
    events = events.replace(/(-orange\.ondrag-bubble\.ondrag-orange\.ondragover-bubble\.ondragover){3,}/g,'$1$1');
    //non-spec-compliant repeats while dragging over fuchsia
    events = events.replace(/(-orange\.ondrag-bubble\.ondrag-pink\.ondragover-bubble\.ondragover){3,}/g,'$1$1');
    events = events.split(/-/g);

    test(function () {
      assert_array_equals(events,

      [/*  1 */ 'orange.onmousedown', //mouse down
        /*  2 */ 'bubble.onmousedown',

        /*  3 */ 'orange.ondragstart', //dragging begins
        /*  4 */ 'bubble.ondragstart',

        /*  5 */ 'orange.ondrag',      //mouse is over orange
        /*  6 */ 'bubble.ondrag',
        /*  7 */ 'orange.ondragenter', //not cancelled
        /*  8 */ 'bubble.ondragenter',
        /*  9 */ 'body.ondragenter',   //so body becomes current target, and the event fires there as well
        /* 10 */ 'body.ondragover',

        /* 11 */ 'orange.ondrag',      //start repeating (some over orange, some over body)
        /* 12 */ 'bubble.ondrag',
        /* 13 */ 'body.ondragover',
        /* 14 */ 'orange.ondrag',      //...twice to make sure it actually repeats
        /* 15 */ 'bubble.ondrag',
        /* 16 */ 'body.ondragover',    //end repeating

        /* 17 */ 'orange.ondrag',      //mouse moves over pink
        /* 18 */ 'bubble.ondrag',
        /* 19 */ 'pink.ondragenter',   //not cancelled
        /* 20 */ 'bubble.ondragenter',
        /* 21 */ 'body.ondragover',    //so body becomes current target, but since it was already the target, dragenter does not need to fire again

        /* 22 */ 'orange.ondrag',      //start repeating (some over pink, some over body)
        /* 23 */ 'bubble.ondrag',
        /* 24 */ 'body.ondragover',
        /* 25 */ 'orange.ondrag',      //...twice to make sure it actually repeats
        /* 26 */ 'bubble.ondrag',
        /* 27 */ 'body.ondragover',    //end repeating

        /* 28 */ 'orange.ondrag',      //mouse moves over yellow
        /* 29 */ 'bubble.ondrag',
        /* 30 */ 'yellow.ondragenter',
        /* 31 */ 'bubble.ondragenter',
        /* 32 */ 'body.ondragleave',
        /* 33 */ 'yellow.ondragover',
        /* 34 */ 'bubble.ondragover',

        /* 35 */ 'orange.ondrag',      //start repeating (over yellow)
        /* 36 */ 'bubble.ondrag',
        /* 37 */ 'yellow.ondragover',
        /* 38 */ 'bubble.ondragover',
        /* 39 */ 'orange.ondrag',      //...twice to make sure it actually repeats
        /* 40 */ 'bubble.ondrag',
        /* 41 */ 'yellow.ondragover',
        /* 42 */ 'bubble.ondragover',  //end repeating

        /* 43 */ 'orange.ondrag',      //mouse moves over body
        /* 44 */ 'bubble.ondrag',
        /* 45 */ 'body.ondragenter',   //not cancelled
        /* 46 */ 'body.ondragenter',   //so it fires again and sets body as current target
        /* 47 */ 'yellow.ondragleave',
        /* 48 */ 'bubble.ondragleave',
        /* 49 */ 'body.ondragover',

        /* 50 */ 'orange.ondrag',      //start repeating (over body)
        /* 51 */ 'bubble.ondrag',
        /* 52 */ 'body.ondragover',
        /* 53 */ 'orange.ondrag',      //...twice to make sure it actually repeats
        /* 54 */ 'bubble.ondrag',
        /* 55 */ 'body.ondragover',    //end repeating

        /* 56 */ 'orange.ondrag',      //mouse moves over blue
        /* 57 */ 'bubble.ondrag',
        /* 58 */ 'blue.ondragenter',
        /* 59 */ 'bubble.ondragenter',
        /* 60 */ 'body.ondragleave',
        /* 61 */ 'blue.ondragover',
        /* 62 */ 'bubble.ondragover',

        /* 63 */ 'orange.ondrag',      //start repeating (over blue)
        /* 64 */ 'bubble.ondrag',
        /* 65 */ 'blue.ondragover',
        /* 66 */ 'bubble.ondragover',
        /* 67 */ 'orange.ondrag',      //...twice to make sure it actually repeats
        /* 68 */ 'bubble.ondrag',
        /* 69 */ 'blue.ondragover',
        /* 70 */ 'bubble.ondragover',  //end repeating

        /* 71 */ 'blue.ondrop',        //release
        /* 72 */ 'bubble.ondrop',
        /* 73 */ 'orange.ondragend',
        /* 74 */ 'bubble.ondragend']

      );
    }, 'Overall sequence');

    /* ondragstart */
    test(function () { assert_true( events.indexOf('orange.ondragstart') != -1 ); }, "orange.ondragstart should fire");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'orange.ondragstart') return e; }).length, 1); }, "orange.ondragstart should fire 1 time");
    test(function () { assert_equals( events[2], 'orange.ondragstart' ); }, "orange.ondragstart should be event handler #3");
    test(function () { assert_equals( events.indexOf('pink.ondragstart'), -1 ); }, "pink.ondragstart should not fire");
    test(function () { assert_equals( events.indexOf('yellow.ondragstart'), -1 ); }, "yellow.ondragstart should not fire");
    test(function () { assert_equals( events.indexOf('blue.ondragstart'), -1 ); }, "blue.ondragstart should not fire");
    test(function () { assert_equals( events.indexOf('body.ondragstart'), -1 ); }, "ondragstart should not fire at the body");
    test(function () { assert_true( events.indexOf('bubble.ondragstart') != -1 ); }, "ondragstart should bubble to body");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'bubble.ondragstart') return e; }).length, 1); }, "ondragstart should only bubble to body 1 time");
    test(function () { assert_equals( events[3], 'bubble.ondragstart' ); }, "ondragstart should bubble to body as event handler #4");

    /* ondrag */
    test(function () { assert_true( events.indexOf('orange.ondrag') != -1 ); }, "orange.ondrag should fire");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'orange.ondrag') return e; }).length, 15); }, "orange.ondrag should fire 15 times");
    for( var i = 0, evindex = [4,10,13,16,21,24,27,34,38,42,49,52,55,62,66]; i < evindex.length; i++ ) {
      test(function () { assert_equals( events[evindex[i]], 'orange.ondrag' ); }, "orange.ondrag should be event handler #"+(evindex[i]+1));
    }
    test(function () { assert_equals( events.indexOf('pink.ondrag'), -1 ); }, "pink.ondrag should not fire");
    test(function () { assert_equals( events.indexOf('yellow.ondrag'), -1 ); }, "yellow.ondrag should not fire");
    test(function () { assert_equals( events.indexOf('blue.ondrag'), -1 ); }, "blue.ondrag should not fire");
    test(function () { assert_equals( events.indexOf('body.ondrag'), -1 ); }, "ondrag should not fire at the body");
    test(function () { assert_true( events.indexOf('bubble.ondrag') != -1 ); }, "ondrag should bubble to body");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'bubble.ondrag') return e; }).length, 15); }, "ondrag should bubble to body 15 times");
    for( var i = 0, evindex = [5,11,14,17,22,25,28,35,39,43,50,53,56,63,67]; i < evindex.length; i++ ) {
      test(function () { assert_equals( events[evindex[i]], 'bubble.ondrag' ); }, "ondrag should bubble to body as event handler #"+(evindex[i]+1));
    }

    /* ondragenter */
    test(function () { assert_true( events.indexOf('orange.ondragenter') != -1 ); }, "orange.ondragenter should fire");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'orange.ondragenter') return e; }).length, 1); }, "orange.ondragenter should fire 1 time");
    test(function () { assert_equals( events[6], 'orange.ondragenter' ); }, "orange.ondragenter should be event handler #7");
    test(function () { assert_true( events.indexOf('pink.ondragenter') != -1 ); }, "pink.ondragenter should fire");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'pink.ondragenter') return e; }).length, 1); }, "pink.ondragenter should fire 1 time");
    test(function () { assert_equals( events[18], 'pink.ondragenter' ); }, "pink.ondragenter should be event handler #19");
    test(function () { assert_true( events.indexOf('yellow.ondragenter') != -1 ); }, "yellow.ondragenter should fire");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'yellow.ondragenter') return e; }).length, 1); }, "yellow.ondragenter should fire 1 time");
    test(function () { assert_equals( events[29], 'yellow.ondragenter' ); }, "yellow.ondragenter should be event handler #30");
    test(function () { assert_true( events.indexOf('blue.ondragenter') != -1 ); }, "blue.ondragenter should fire");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'blue.ondragenter') return e; }).length, 1); }, "blue.ondragenter should fire 1 time");
    test(function () { assert_equals( events[57], 'blue.ondragenter' ); }, "blue.ondragenter should be event handler #58");
    test(function () { assert_true( events.indexOf('body.ondragenter') != -1 ); }, "ondragenter should fire at body");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'body.ondragenter') return e; }).length, 3); }, "ondragenter should fire at body 2 times");
    for( var i = 0, evindex = [8,44,45]; i < evindex.length; i++ ) {
      test(function () { assert_equals( events[evindex[i]], 'body.ondragenter' ); }, "ondragenter should fire at body as event handler #"+(evindex[i]+1));
    }
    test(function () { assert_true( events.indexOf('bubble.ondragenter') != -1 ); }, "ondragenter should bubble to body");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'bubble.ondragenter') return e; }).length, 4); }, "ondragenter should bubble to body 4 times");
    for( var i = 0, evindex = [7,19,30,58]; i < evindex.length; i++ ) {
      test(function () { assert_equals( events[evindex[i]], 'bubble.ondragenter' ); }, "ondragenter should bubble to body as event handler #"+(evindex[i]+1));
    }

    /* ondragover */
    test(function () { assert_equals( events.indexOf('orange.ondragover'), -1 ); }, "orange.ondragover should not fire");
    test(function () { assert_equals( events.indexOf('pink.ondragover'), -1 ); }, "pink.ondragover should not fire");
    test(function () { assert_true( events.indexOf('yellow.ondragover') != -1 ); }, "yellow.ondragover should fire");
    test(function () { assert_equals( events.filter(function (e) { if (e == 'yellow.ondragover') return e; }).length, 3); }, "yellow.ondragover should fire 3 times");
    f
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 150,
        "byte_start": 87,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 229,
        "byte_start": 160,
        "col": 1,
        "line": 4
      }
    },
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 262,
        "byte_start": 239,
        "col": 1,
        "line": 5
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
  "source_name": "html/editing/dnd/events/events-suite-manual.html"
}
```
