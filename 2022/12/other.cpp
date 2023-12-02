#include <iostream>
#include <fstream>
#include <string>
#include <queue>

#define GRID_W 113
#define GRID_H 41

//define PART_2 to solve the second problem.

#define GRID_S GRID_W * GRID_H

using namespace std;

char grid[GRID_S] = {};
bool isSet[GRID_S] = {};
bool isDone[GRID_S] = {};
int distances[GRID_S] = {};
typedef pair<int, int> iPair;
priority_queue<iPair, vector<iPair>, greater<iPair>> pq;


void checkPoint(iPair offset, iPair parent) {
	int index = (parent.second + offset.second) * GRID_W + parent.first + offset.first;
	int pIndex = parent.second* GRID_W + parent.first;
	char c = grid[index];
	char pc = grid[pIndex];
	if (c > pc + 1 || (pc != 'z' && c == 'E')) return;
	if (!isSet[index] && !isDone[index] || !isDone[index] && isSet[index] && distances[pIndex] + 1 < distances[index]) {
		distances[index] = distances[pIndex] + 1;
		isSet[index] = true;
		pq.push({ distances[index], index });
	}
}

int main()
{
	ifstream input("input.txt");
	if (!input.is_open()) return -23;
	string line;
	int row = 0;
	int start = 0;
	while (getline(input, line)) {
		
		for (int i = 0; i < GRID_W; i++) {
			grid[row * GRID_W + i] = line.at(i);
#ifdef PART_2
			if (line.at(i) == 'a') {
				pq.push({ 0,row * GRID_W + i });
				distances[row * GRID_W + i] = 0;
				isSet[row * GRID_W + i] = true;
			}
#endif
			if (line.at(i) == 'S') {
				start = row * GRID_W + i;
				grid[row * GRID_W + i] = 'a';
			}
			
		}
		row++;
	}
	iPair current = { 0,start };
	
	while (true) {
		cout << "Please wait...\n \n";
		int x = current.second % GRID_W;
		int y = current.second / GRID_W;
		if (grid[current.second] == 'E') {
			cout << "The shortest distance is " << distances[current.second] << ".\n";
			break;
		}
		if (x > 0) {
			checkPoint({ -1,0 }, { x,y });
		}
		if (x < GRID_W - 1) {
			checkPoint( { 1,0}, { x,y });
		}
		if (y > 0) {
			checkPoint({ 0,-1 }, { x,y });
		}
		if (y < GRID_H - 1) {
			checkPoint({ 0,1 }, { x,y });
		}
		isDone[current.second] = true;
		if (pq.empty()) return -24;
		current = pq.top();
		pq.pop();
	}

}
