import re
class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        chars = list(s)
        p.replace(".", "[a-z]")
        print(p)
        print(re.fullmatch(p, s, re.IGNORECASE))
        if re.fullmatch(p, s, re.IGNORECASE) == None:
            return False
        return True
         
        