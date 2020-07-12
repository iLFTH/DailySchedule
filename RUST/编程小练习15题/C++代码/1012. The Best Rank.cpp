#include<stdio.h>
#include<string.h>
#include<iostream>
#include<algorithm>
using namespace std; 
struct Student{
    int id;
    int grade[4];  //0-A 1-C 2-M 3-E 
}stu[30010];



char course[4]={'A','C','M','E'};  //按照优先级排列 
int Rank[10000000][4]={0};  //id及其各科的排名
int now;  //判断是哪一科 

bool cmp(Student a,Student b){
    return a.grade[now]>b.grade[now];
}

int main(){
    int m,n;
    cin>>n;
    cin>>m;
    for(int i=0;i<n;i++){
        scanf("%d %d %d %d",&stu[i].id,&stu[i].grade[1],&stu[i].grade[2],&stu[i].grade[3]);
        stu[i].grade[0]=(stu[i].grade[1]+stu[i].grade[2]+stu[i].grade[3]);  //A
    }
    for(now=0;now<4;now++){  //将每一门的排名都求出来 
        sort(stu,stu+n,cmp);
        Rank[stu[0].id][now]=1;
        for(int i=1;i<n;i++){
            if(stu[i].grade[now]==stu[i-1].grade[now]){
                Rank[stu[i].id][now]=Rank[stu[i-1].id][now];
            }
            else{
                Rank[stu[i].id][now]=i+1;
            }
        }
    }
    int query;  //查询哪些学生要显示 
    for(int i=0;i<m;i++){
        cin>>query;
        if(Rank[query][0]==0){  //如果学号不存在 
            cout<<"N/A"<<endl;
        }
        else{
            int k=0;
            for(int j=0;j<4;j++){  //遍历找到最优排名 
                if(Rank[query][j]<Rank[query][k]){
                    k=j;
                }
            }
            cout<<Rank[query][k]<<" "<<course[k]<<endl;
        }
    }
    return 0;
}