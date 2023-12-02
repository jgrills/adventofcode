if ! { [[ -n $ZSH_EVAL_CONTEXT && $ZSH_EVAL_CONTEXT =~ :file$ ]] || { [[ -n $BASH_VERSION ]] && (return 0 2>/dev/null); }; }; then
    echo "You must source this script"
    exit 1
fi

if [[ "${BASH_SOURCE[0]}" != *"/"* ]]; then
  echo "You must specify the path to this script"
  false
else

  function b() { ./b.sh "$@"; }
  function r() { ./r.sh "$@"; }
  function bnr() { ./bnr.sh "$@"; }

  mkdir -p .vscode
  [[ ! -e .vscode/launch.json ]] && cp launch.json .vscode
  true
fi
