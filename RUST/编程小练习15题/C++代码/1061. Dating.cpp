#include <cstdio>
#include <iostream>
#include <cstring>
#include <cctype>
#include <map>

using namespace std;

char a1[80],a2[80],a3[80],a4[80];
string day[7] = {"MON ","TUE ","WED ","THU ","FRI ","SAT ","SUN "};

int main(){

    scanf("%s%s%s%s",a1,a2,a3,a4);

    int l1 = strlen(a1), l2 = strlen(a2), l3 = strlen(a3), l4 = strlen(a4);
    int count = 0, c = 0;
    char a, b;
    for(int i=0; i<min(l1,l2); i++){
        if(a1[i]==a2[i]&&(a1[i] >= 'A' && a1[i] <= 'G')){
            count = i;
            a = a1[i];
            break;
        }
    }
    for(int i=count+1; i<min(l1,l2); i++){
        if(a1[i]==a2[i]&&((a1[i] >= 'A' && a1[i] <= 'N') || isdigit(a1[i]))){
            b = a1[i];
            break;
        }
    }
    for(int i=0; i<min(l3,l4); i++){
        if(a3[i]==a4[i] && isalpha(a3[i])) {
            c = i;
            break;
        }
    }

    cout << day[a-'A'];
    int m = b - '0';
    if(!isdigit(b)) m = b - 'A' + 10;
    printf("%02d:%02d",m,c);

    return 0;
}
